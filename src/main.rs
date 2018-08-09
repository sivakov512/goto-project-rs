extern crate goto_project;

use goto_project::manager::Manager;

fn main() {
    let manager = Manager::new(".goto-project.yaml");
    manager.open_project("py_goto_project");
}
