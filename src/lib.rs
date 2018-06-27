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

    pub fn find(name: &str) -> Result<Config, String> {
        let mut path: PathBuf = env::home_dir().ok_or("Home dir not found on your system")?;
        path.push(name);

        let path_str = path.to_str().unwrap();
        match path.exists() {
            true => Ok(Config::new(path_str)),
            false => Err(format!("\"{}\" config not found", path_str)),
        }
    }

    fn parse(&self) -> Projects {
        let mut file = File::open(&self.path).unwrap();
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
    fn find_returns_error_if_nothing_found() {
        let result = Config::find("nonexistence_config.yaml");

        let err = result.err().unwrap();
        assert!(err.contains("nonexistence_config.yaml\" config not found"));
    }

    #[test]
    fn find_returns_correct_config_if_found() {
        let _fake_config = FakeConfig::new("conf1.yaml", CONFIG_CONTENT);

        let result = Config::find("conf1.yaml");

        let config = result.unwrap();
        assert!(config.path.contains("conf1.yaml"));
    }

    #[test]
    #[should_panic]
    fn invalid_config_parsing_should_panic() {
        let _fake_config = FakeConfig::new("conf3.yaml", "lolkek");
        let config = Config::find("conf3.yaml").unwrap();

        config.parse();
    }

    #[test]
    fn parsed_project_with_path_only() {
        let _fake_config = FakeConfig::new("conf4.yaml", CONFIG_CONTENT);
        let config = Config::find("conf4.yaml").unwrap();

        let projects = config.parse();
        let project = &projects["awesome-project"];

        assert_eq!(project.path, "~/Devel/Projects/awesome-project/");
        assert_eq!(project.instructions.len(), 0);
    }

    #[test]
    fn parsed_project_with_all_data() {
        let _fake_config = FakeConfig::new("conf5.yaml", CONFIG_CONTENT);
        let config = Config::find("conf5.yaml").unwrap();

        let projects = config.parse();
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
