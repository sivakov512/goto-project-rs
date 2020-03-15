#![allow(dead_code)]
use serde_derive::Deserialize;
use std::env;
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
}

#[cfg(test)]
mod tests {
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
