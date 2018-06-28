use config::ConfigLoader;
use serde_yaml;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
pub struct Project {
    path: String,
    #[serde(default)]
    instructions: Vec<String>,
}

pub type Projects = BTreeMap<String, Project>;

pub trait ProjectsParser {
    fn parse(config: &ConfigLoader) -> Projects;
}

impl ProjectsParser for Projects {
    fn parse(config: &ConfigLoader) -> Projects {
        let contents = &config.load();
        serde_yaml::from_str(&contents).unwrap()
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
    fn project_with_all_data_parsed_correctly() {
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

        let projects = Projects::parse(&fake_config);
        let project: &Project = &projects["awesome-project"];

        assert_eq!(project.path, "~/Devel/Projects/awesome-project/");
        assert_eq!(project.instructions.len(), 0);
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
