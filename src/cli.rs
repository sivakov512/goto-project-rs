use crate::manager::Manager;
use clap::{crate_authors, crate_description, crate_version};
use clap::{App, Arg};
use dirs;

fn build_cli<'a>(project_list: &[&'a str]) -> App<'a, 'a> {
    App::new("Goto project")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .after_help(
            "Before usage write configuration of your projects list in ~/.goto-project.yaml",
        )
        .arg(
            Arg::with_name("project")
                .takes_value(true)
                .possible_values(project_list)
                .help("Open choosen project if passed, otherwise list them all."),
        )
        .arg(Arg::with_name("subpath").takes_value(true))
        .arg(Arg::with_name("list-subdirs").long("list-subdirs"))
}

pub fn run_cli() {
    let fpath = dirs::home_dir().unwrap().join(".goto-project.yaml");
    let manager = Manager::from_config_file(fpath.to_str().unwrap());

    let project_list = manager.list_projects();
    let project_list: Vec<&str> = project_list
        .iter()
        .map(std::convert::AsRef::as_ref)
        .collect();

    let matches = build_cli(&project_list).get_matches();

    match matches.value_of("project") {
        Some(project_name) => {
            let mut project = manager.get_project(project_name).clone();

            if let Some(subpath) = matches.value_of("subpath") {
                project = project.goto_subdir(subpath);
            }

            if matches.is_present("list-subdirs") {
                for subdir_name in project.list_subdirs() {
                    println!("{}", subdir_name);
                }
            } else {
                project.open()
            }
        }
        None => {
            for project_name in project_list {
                println!("{}", project_name);
            }
        }
    }
}
