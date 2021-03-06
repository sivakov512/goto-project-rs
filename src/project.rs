#![allow(dead_code)]
use dirs;
use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::path::Path;
use subprocess::Exec;

#[derive(Deserialize, Clone)]
pub struct Project {
    pub path: String,
    #[serde(default)]
    pub instructions: Vec<String>,
}

impl Project {
    fn opening_command(&self) -> String {
        let mut command_parts: Vec<String> = vec![format!("cd {}", self.path)];
        command_parts.extend_from_slice(&self.instructions);
        command_parts.extend_from_slice(&[env::var("SHELL").unwrap(), "clear".to_owned()]);

        command_parts.join(" && ")
    }

    pub fn list_subdirs(&self) -> Vec<String> {
        let mut path = self.path.clone();
        if self.path.starts_with('~') {
            path = path.replace('~', dirs::home_dir().unwrap().to_str().unwrap());
        }

        let mut subdirs: Vec<String> = fs::read_dir(path)
            .unwrap()
            .map(|r| r.unwrap())
            .filter(|e| e.metadata().unwrap().is_dir())
            .map(|e| e.file_name().into_string().unwrap())
            .collect();
        subdirs.sort();
        subdirs
    }

    pub fn goto_subdir(self, subdir: &str) -> Self {
        let path = Path::new(&self.path)
            .join(subdir)
            .to_str()
            .unwrap()
            .to_owned();
        Self { path, ..self }
    }

    pub fn open(&self) {
        Exec::shell(self.opening_command()).join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use uuid::Uuid;

    mod opening_command {
        use super::*;

        #[test]
        fn returns_expected_for_project_without_instructions() {
            let project = Project {
                path: "/tmp/goto/example".to_owned(),
                instructions: vec![],
            };

            let got = project.opening_command();

            assert_eq!(
                got,
                format!(
                    "cd /tmp/goto/example && {} && clear",
                    env::var("SHELL").unwrap()
                )
            )
        }

        #[test]
        fn returns_expected_for_project_with_instructions() {
            let project = Project {
                path: "/tmp/goto/example".to_owned(),
                instructions: vec!["call_something".to_owned(), "source /tmp/stuff".to_owned()],
            };

            let got = project.opening_command();

            assert_eq!(
                got,
                format!(
                    "cd /tmp/goto/example && call_something && source /tmp/stuff && {} && clear",
                    env::var("SHELL").unwrap()
                )
            )
        }
    }

    mod list_subdirs {
        use super::*;

        #[test]
        fn returns_nothing_if_project_has_no_subdirs() {
            let c = TmpDir::new();
            let project = Project {
                path: c.path(),
                instructions: vec![],
            };

            let got = project.list_subdirs();

            assert_eq!(got.len(), 0);
        }

        #[test]
        fn returns_subdir_names() {
            let c = TmpDir::new().with_subdirs(&["sub0", "sub1", "sub2"]);
            let project = Project {
                path: c.path(),
                instructions: vec![],
            };

            let got = project.list_subdirs();

            assert_eq!(got, vec!["sub0", "sub1", "sub2"]);
        }

        #[test]
        fn not_returns_filenames() {
            let c = TmpDir::new().with_files(&["file0.txt", "file1.txt", "file2.txt"]);
            let project = Project {
                path: c.path(),
                instructions: vec![],
            };

            let got = project.list_subdirs();

            assert_eq!(got.len(), 0);
        }

        #[test]
        fn not_panics_for_tilde() {
            let project = Project {
                path: "~".to_owned(),
                instructions: vec![],
            };

            project.list_subdirs();
        }
    }

    mod goto_subdir {
        use super::*;

        #[test]
        fn creates_project_with_extended_path() {
            let project = Project {
                path: "/tmp/goto/example".to_owned(),
                instructions: vec!["call_something".to_owned(), "source /tmp/stuff".to_owned()],
            };

            let got = project.goto_subdir("subdir");

            assert_eq!(got.path, "/tmp/goto/example/subdir");
            assert_eq!(
                got.instructions,
                vec!["call_something", "source /tmp/stuff"]
            );
        }
    }

    struct TmpDir {
        path: PathBuf,
    }

    impl TmpDir {
        fn new() -> Self {
            let path = env::temp_dir()
                .join("goto_project")
                .join(Uuid::new_v4().to_string());
            fs::create_dir_all(&path).unwrap();
            let creator = TmpDir { path };
            creator
        }

        fn with_subdirs(self, subdirs: &[&str]) -> Self {
            for subdir in subdirs.iter() {
                let path = self.path.join(subdir);
                fs::create_dir(&path).unwrap();
            }
            self
        }

        fn with_files(self, fnames: &[&str]) -> Self {
            for fname in fnames.iter() {
                let path = self.path.join(fname);
                fs::File::create(path).unwrap();
            }
            self
        }

        fn path(&self) -> String {
            self.path.to_str().unwrap().to_owned()
        }
    }

    impl Drop for TmpDir {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.path).unwrap();
        }
    }
}
