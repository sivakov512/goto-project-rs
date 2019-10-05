use crate::config::ConfigLoader;
use serde_derive::Deserialize;
use serde_yaml;
use std::collections::BTreeMap;
use std::env;
use subprocess::Exec;

#[derive(Deserialize, Debug)]
pub struct Project {
    path: String,
    #[serde(default)]
    instructions: Vec<String>,
}

impl Project {
    fn command_string(&self) -> String {
        let mut command_parts: Vec<String> = vec![];

        command_parts.push(format!("cd {}", &self.path));
        command_parts.extend(self.instructions.clone());

        command_parts.push(env::var("SHELL").unwrap());
        command_parts.push(String::from("clear"));

        command_parts.join(" && ")
    }

    pub fn open(&self) {
        Exec::cmd(env::var("SHELL").unwrap())
            .arg("-c")
            .arg(self.command_string())
            .join()
            .unwrap();
    }
}

pub type Projects = BTreeMap<String, Project>;

pub trait ProjectsParser {
    fn parse(config: &dyn ConfigLoader) -> Projects;
}

impl ProjectsParser for Projects {
    fn parse(config: &dyn ConfigLoader) -> Projects {
        let contents = &config.load();
        serde_yaml::from_str(&contents).unwrap()
    }
}

pub trait ProjectsListing {
    fn list(&self) -> Vec<String>;
}

impl ProjectsListing for Projects {
    fn list(&self) -> Vec<String> {
        let mut list: Vec<String> = vec![];
        for name in self.keys() {
            list.push(name.clone())
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_returns_all_founded_projects() {
        let fake_config = FakeConfig::new(CONFIG_CONTENT);

        let projects = Projects::parse(&fake_config);

        assert_eq!(projects.len(), 2);
    }

    #[test]
    #[should_panic]
    fn parse_should_panic_on_invalid_content() {
        let fake_config = FakeConfig::new("azaza");

        Projects::parse(&fake_config);
    }

    #[test]
    fn list_returns_all_project_names() {
        let fake_config = FakeConfig::new(CONFIG_CONTENT);
        let projects = Projects::parse(&fake_config);

        let project_names = projects.list();

        assert_eq!(
            project_names,
            vec!["awesome-project", "yet_another_project"]
        );
    }

    #[test]
    fn project_with_instructions_parsed_correctly() {
        let fake_config = FakeConfig::new(CONFIG_CONTENT);

        let projects = Projects::parse(&fake_config);
        let project: &Project = &projects["yet_another_project"];

        assert_eq!(project.path, "~/Devel/Projects/yet_another_project");
        assert_eq!(
            project.instructions,
            vec![
                "source ~/Devel/Envs/yet_another_project/bin/activate",
                "export FLASK_APP=app.py",
                "export FLASK_DEBUG=1",
            ]
        )
    }

    #[test]
    fn project_without_instructions_parsed_correctly() {
        let fake_config = FakeConfig::new(CONFIG_CONTENT);

        let projects: Projects = Projects::parse(&fake_config);
        let project: &Project = &projects["awesome-project"];

        assert_eq!(project.path, "~/Devel/Projects/awesome-project/");
        assert_eq!(project.instructions.len(), 0);
    }

    #[test]
    fn command_string_for_project_without_instructions() {
        let shell = env::var("SHELL").unwrap();

        let project = Project {
            path: String::from("/path/"),
            instructions: vec![],
        };

        assert_eq!(
            project.command_string(),
            format!("cd /path/ && {} && clear", shell)
        );
    }

    #[test]
    fn command_string_for_project_with_instructions() {
        let shell = env::var("SHELL").unwrap();

        let project = Project {
            path: String::from("/some/path/"),
            instructions: vec!["call_something".to_owned(), "source /stuff".to_owned()],
        };

        assert_eq!(
            project.command_string(),
            format!(
                "cd /some/path/ && call_something && source /stuff && {} && clear",
                shell
            )
        );
    }

    const CONFIG_CONTENT: &str = "
awesome-project:
  path: ~/Devel/Projects/awesome-project/

yet_another_project:
  path: ~/Devel/Projects/yet_another_project
  instructions:
    - source ~/Devel/Envs/yet_another_project/bin/activate
    - export FLASK_APP=app.py
    - export FLASK_DEBUG=1
";

    struct FakeConfig {
        contents: String,
    }

    impl FakeConfig {
        fn new(contents: &str) -> FakeConfig {
            let contents = contents.to_owned();
            FakeConfig { contents }
        }
    }

    impl ConfigLoader for FakeConfig {
        fn load(&self) -> String {
            self.contents.clone()
        }
    }
}
