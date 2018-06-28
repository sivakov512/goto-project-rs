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
}

impl Config {
    pub fn new(name: &str) -> Config {
        let mut path: PathBuf = env::home_dir().unwrap();
        path.push(name);

        let path_str = path.to_str().unwrap();
        let path = match path.exists() {
            true => path_str.to_owned(),
            false => panic!(format!("\"{}\" config not found", path_str)),
        };

        Config { path }
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
    fn new_returns_correct_config() {
        let fake_config = FakeConfig::new("conf6.yaml", CONFIG_CONTENT);

        let config = Config::new("conf6.yaml");

        assert_eq!(&config.path, &fake_config.path);
    }

    #[test]
    #[should_panic]
    fn new_should_panic_if_config_file_not_found() {
        Config::new("nonexistence_config.yaml");
    }

    #[test]
    #[should_panic]
    fn invalid_config_parsing_should_panic() {
        let _fake_config = FakeConfig::new("conf3.yaml", "lolkek");
        let config = Config::new("conf3.yaml");

        config.parse();
    }

    #[test]
    fn parse_returns_all_founded_projects() {
        let _fake_config = FakeConfig::new("conf4.yaml", CONFIG_CONTENT);
        let config = Config::new("conf4.yaml");

        let projects = config.parse();

        assert_eq!(projects.len(), 2);
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
