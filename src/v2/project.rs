#![allow(dead_code)]
use serde_derive::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub instructions: Vec<String>,
}

impl Project {
    fn opening_command(&self) -> String {
        let mut command_parts: Vec<String> = vec![format!("cd {}", self.path)];
        command_parts.extend_from_slice(&self.instructions);
        command_parts.extend_from_slice(&[env::var("SHELL").unwrap(), "clear".to_owned()]);

        command_parts.join(" && ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn returns_expected_for_project_without_instructions() {
        let project = Project {
            name: "Example".to_owned(),
            path: "/tmp/goto/example/".to_owned(),
            instructions: vec![],
        };

        let got = project.opening_command();

        assert_eq!(
            got,
            format!(
                "cd /tmp/goto/example/ && {} && clear",
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

        let got = project.opening_command();

        assert_eq!(
            got,
            format!(
                "cd /tmp/goto/example/ && call_something && source /tmp/stuff && {} && clear",
                env::var("SHELL").unwrap()
            )
        )
    }
}
