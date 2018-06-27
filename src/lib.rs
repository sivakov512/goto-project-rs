#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
struct Project {
    path: String,
    #[serde(default)]
    instructions: Vec<String>,
}

type Projects = BTreeMap<String, Project>;

pub struct Config {
    path: String,
    projects: Option<BTreeMap<String, Project>>,
}

impl Config {
    fn new(path: &str) -> Config {
        let path = path.to_owned();
        Config {
            path,
            projects: None,
        }
    }

    fn find_path(name: &str) -> String {
        let mut path: PathBuf = env::home_dir().unwrap();
        path.push(name);

        let path_str = path.to_str().unwrap();
        match path.exists() {
            true => path_str.to_owned(),
            false => panic!(format!("\"{}\" config not found", path_str)),
        }
    }

    fn parse(path: &str) -> Projects {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        serde_yaml::from_str(&contents).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{remove_file, File};

    #[test]
    #[should_panic]
    fn find_path_should_panic_if_nothing_found() {
        Config::find_path("nonexistence_config.yaml");
    }

    #[test]
    fn find_returns_correct_config_if_found() {
        let _fake_config = FakeConfig::new("conf1.yaml", CONFIG_CONTENT);

        let result = Config::find_path("conf1.yaml");

        assert!(result.contains("conf1.yaml"));
    }

    #[test]
    #[should_panic]
    fn invalid_config_parsing_should_panic() {
        let fake_config = FakeConfig::new("conf3.yaml", "lolkek");

        Config::parse(&fake_config.path);
    }

    #[test]
    fn parsed_project_with_path_only() {
        let fake_config = FakeConfig::new("conf4.yaml", CONFIG_CONTENT);

        let projects = Config::parse(&fake_config.path);
        let project = &projects["awesome-project"];

        assert_eq!(project.path, "~/Devel/Projects/awesome-project/");
        assert_eq!(project.instructions.len(), 0);
    }

    #[test]
    fn parsed_project_with_all_data() {
        let fake_config = FakeConfig::new("conf5.yaml", CONFIG_CONTENT);

        let projects = Config::parse(&fake_config.path);
        let project = &projects["yet_another_project"];

        assert_eq!(project.path, "~/Devel/Projects/yet_another_project");
        assert_eq!(
            project.instructions,
            vec![
                "source ~/Devel/Envs/yet_another_project/bin/activate",
                "export FLASK_APP=app.py",
                "export FLASK_DEBUG=1",
            ]
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
        path: String,
    }

    impl FakeConfig {
        fn new(name: &str, contents: &str) -> FakeConfig {
            let mut path: PathBuf = env::home_dir().unwrap();
            path.push(name);

            let path = path.to_str().unwrap();
            let mut file = File::create(path).unwrap();

            file.write_all(contents.as_bytes()).unwrap();
            file.flush().unwrap();

            FakeConfig {
                path: path.to_owned(),
            }
        }
    }

    impl Drop for FakeConfig {
        fn drop(&mut self) {
            let path = self.path.clone();
            remove_file(path).expect(
                format!("Something went wront when removing test file \"{}\"!", "a").as_str(),
            );
        }
    }
}
