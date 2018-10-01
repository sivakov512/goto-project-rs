use config::Config;
use projects::{Projects, ProjectsListing, ProjectsParser};

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
        self.projects[name].open();
    }

    pub fn list_projects(&self) -> Vec<String> {
        self.projects.list()
    }
}
