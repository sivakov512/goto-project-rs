use config::Config;
use projects::{Projects, ProjectsParser, ProjectsListing};

pub struct Manager {
    projects: Projects,
}

impl Manager {
    pub fn new(config_name: &str) -> Manager {
        let config = Config::new(config_name);
        let projects = Projects::parse(&config);
        Manager { projects }
    }

    pub fn open_project(&self, name: &str) {
        let project = self.projects.get(name).unwrap();
        project.open();
    }

    pub fn list_projects(&self) -> Vec<String> {
        self.projects.list()
    }
}
