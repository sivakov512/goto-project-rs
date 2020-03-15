#![allow(dead_code)]
use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub instructions: Vec<String>,
}

impl Project {
    fn opening_command(&self, subdir: Option<&str>) -> String {
        let path = match subdir {
            Some(subdir) => Path::new(&self.path).join(subdir),
            None => PathBuf::from(&self.path),
        };

        let mut command_parts: Vec<String> = vec![format!("cd {}", path.to_str().unwrap())];
        command_parts.extend_from_slice(&self.instructions);
        command_parts.extend_from_slice(&[env::var("SHELL").unwrap(), "clear".to_owned()]);

        command_parts.join(" && ")
    }

    fn list_subdirs(&self) -> Vec<String> {
        let mut subdirs: Vec<String> = fs::read_dir(&self.path)
            .unwrap()
            .map(|r| r.unwrap())
            .filter(|e| e.metadata().unwrap().is_dir())
            .map(|e| e.file_name().into_string().unwrap())
            .collect();
        subdirs.sort();
        subdirs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod opening_command {
        use super::*;

        #[test]
        fn returns_expected_for_project_without_instructions() {
            let project = Project {
                name: "Example".to_owned(),
                path: "/tmp/goto/example/".to_owned(),
                instructions: vec![],
            };

            let got = project.opening_command(None);

            assert_eq!(
                got,
                format!(
                    "cd /tmp/goto/example/ && {} && clear",
                    env::var("SHELL").unwrap()
                )
            )
        }

        #[test]
        fn returns_expected_for_project_without_instructions_but_with_subdir() {
            let project = Project {
                name: "Example".to_owned(),
                path: "/tmp/goto/example/".to_owned(),
                instructions: vec![],
            };

            let got = project.opening_command(Some("subsub"));

            assert_eq!(
                got,
                format!(
                    "cd /tmp/goto/example/subsub && {} && clear",
                    env::var("SHELL").unwrap()
                )
            )
        }

        #[test]
        fn returns_expected_for_project_with_instructions() {
            let project = Project {
                name: "Example".to_owned(),
                path: "/tmp/goto/example/".to_owned(),
                instructions: vec!["call_something".to_owned(), "source /tmp/stuff".to_owned()],
            };

            let got = project.opening_command(None);

            assert_eq!(
                got,
                format!(
                    "cd /tmp/goto/example/ && call_something && source /tmp/stuff && {} && clear",
                    env::var("SHELL").unwrap()
                )
            )
        }

        #[test]
        fn returns_expected_for_project_with_instructions_and_subdir() {
            let project = Project {
                name: "Example".to_owned(),
                path: "/tmp/goto/example/".to_owned(),
                instructions: vec!["call_something".to_owned(), "source /tmp/stuff".to_owned()],
            };

            let got = project.opening_command(Some("subsub"));

            assert_eq!(
                got,
                format!(
                    "cd /tmp/goto/example/subsub && call_something && source /tmp/stuff && {} && clear",
                    env::var("SHELL").unwrap()
                    )
                )
        }
    }

    mod list_subdirs {
        use super::*;

        #[test]
        fn returns_nothing_if_project_has_no_subdirs() {
            let c = DirCreator::new("dir0");
            let project = Project {
                name: "Example".to_owned(),
                path: c.path(),
                instructions: vec![],
            };

            let got = project.list_subdirs();

            assert_eq!(got.len(), 0);
        }

        #[test]
        fn returns_subdir_names() {
            let c = DirCreator::new("dir1");
            c.create_subdirs(&["sub0", "sub1", "sub2"]);
            let project = Project {
                name: "Example".to_owned(),
                path: c.path(),
                instructions: vec![],
            };

            let got = project.list_subdirs();

            assert_eq!(
                got,
                vec!["sub0".to_owned(), "sub1".to_owned(), "sub2".to_owned()]
            );
        }

        #[test]
        fn not_returns_filenames() {
            let c = DirCreator::new("dir2");
            c.create_files(&["file0.txt", "file1.txt", "file2.txt"]);
            let project = Project {
                name: "Example".to_owned(),
                path: c.path(),
                instructions: vec![],
            };

            let got = project.list_subdirs();

            assert_eq!(got.len(), 0);
        }
    }

    struct DirCreator {
        path: PathBuf,
    }

    impl DirCreator {
        fn new(path: &str) -> Self {
            let path = env::temp_dir().join("goto_project").join(path);
            fs::create_dir_all(&path).unwrap();
            let creator = DirCreator { path };
            creator
        }

        fn path(&self) -> String {
            self.path.to_str().unwrap().to_owned()
        }

        fn create_subdirs(&self, subdirs: &[&str]) {
            for subdir in subdirs.iter() {
                let path = self.path.join(subdir);
                fs::create_dir(&path).unwrap();
            }
        }

        fn create_files(&self, fnames: &[&str]) {
            for fname in fnames.iter() {
                let path = self.path.join(fname);
                fs::File::create(path).unwrap();
            }
        }
    }

    impl Drop for DirCreator {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.path).unwrap();
        }
    }
}
