#![allow(dead_code)]
use crate::v2::project::Project;
use serde_yaml;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct Manager {
    projects: BTreeMap<String, Project>,
}

impl Manager {
    pub fn from_config_file(config_path: &str) -> Self {
        let fpath = PathBuf::from(config_path);
        if !&fpath.exists() {
            panic!("Config file at \"{}\" not found.", fpath.display())
        }

        let mut file = File::open(&fpath).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        Self {
            projects: serde_yaml::from_str(&contents).unwrap(),
        }
    }

    pub fn list_projects(&self) -> Vec<String> {
        self.projects.keys().cloned().collect()
    }

    pub fn get_project(&self, name: &str) -> &Project {
        &self.projects[name]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::io::Write;

    mod from_config_file {
        use super::*;

        #[test]
        fn project_without_instructions_parsed_correctly() {
            let c = ConfigFile::new("example0.yaml", CONFIG_CONTENT);
            let manager = Manager::from_config_file(&c.fpath);

            let project = &manager.projects["awesome-project"];

            assert_eq!(project.path, "~/Devel/Projects/awesome-project/");
            assert_eq!(project.instructions.len(), 0);
        }

        #[test]
        fn project_with_instruction_parsed_correctly() {
            let c = ConfigFile::new("example1.yaml", CONFIG_CONTENT);
            let manager = Manager::from_config_file(&c.fpath);

            let project = &manager.projects["yet_another_project"];

            assert_eq!(project.path, "~/Devel/Projects/yet_another_project");
            assert_eq!(
                project.instructions,
                vec![
                    "source ~/Devel/Envs/yet_another_project/bin/activate".to_owned(),
                    "export FLASK_APP=app.py".to_owned(),
                    "export FLASK_DEBUG=1".to_owned(),
                ]
            )
        }

        #[test]
        fn parses_all_defined_projects() {
            let c = ConfigFile::new("example2.yaml", CONFIG_CONTENT);

            let manager = Manager::from_config_file(&c.fpath);

            assert_eq!(manager.projects.len(), 2);
        }

        #[test]
        #[should_panic(expected = "Config file at \"/tmp/lol/kek.yaml\" not found.")]
        fn will_panic_if_config_does_not_exists() {
            Manager::from_config_file("/tmp/lol/kek.yaml");
        }

        #[test]
        #[should_panic]
        fn will_panic_for_wrong_config() {
            let c = ConfigFile::new("example3.yaml", "awesome: kek");

            Manager::from_config_file(&c.fpath);
        }
    }

    mod list_projects {
        use super::*;

        #[test]
        fn returns_names_for_all_defined_projects() {
            let c = ConfigFile::new("example4.yaml", CONFIG_CONTENT);
            let manager = Manager::from_config_file(&c.fpath);

            let got = manager.list_projects();

            assert_eq!(got, vec!["awesome-project", "yet_another_project"]);
        }

        #[test]
        fn returns_empty_vector_if_no_projects_defined() {
            let manager = Manager {
                projects: BTreeMap::default(),
            };

            let got = manager.list_projects();

            assert_eq!(got.len(), 0);
        }
    }

    mod get_project {
        use super::*;

        #[test]
        fn returns_project() {
            let c = ConfigFile::new("example5.yaml", CONFIG_CONTENT);
            let manager = Manager::from_config_file(&c.fpath);

            let got = manager.get_project("awesome-project");

            assert_eq!(got.path, "~/Devel/Projects/awesome-project/");
            assert_eq!(got.instructions.len(), 0);
        }

        #[test]
        #[should_panic]
        fn panics_if_project_not_found() {
            let manager = Manager {
                projects: BTreeMap::default(),
            };

            manager.get_project("awesome-project");
        }
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

    struct ConfigFile {
        fpath: String,
    }

    impl ConfigFile {
        fn new(fname: &str, contents: &str) -> Self {
            let path = env::temp_dir();
            fs::create_dir_all(&path).unwrap();

            let fpath = path.join(fname);
            let mut file = File::create(&fpath).unwrap();
            file.write_all(contents.as_bytes()).unwrap();

            Self {
                fpath: fpath.to_str().unwrap().to_owned(),
            }
        }
    }

    impl Drop for ConfigFile {
        fn drop(&mut self) {
            fs::remove_file(&self.fpath).unwrap();
        }
    }
}
