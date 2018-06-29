#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate subprocess;

mod config;
mod manager;
mod projects;

use manager::Manager;

fn main() {
    let manager = Manager::new(".goto-project.yaml");
    manager.open_project("py_goto_project");
}
