#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate dirs;
extern crate serde_yaml;
extern crate subprocess;

pub mod cli;
mod config;
mod manager;
mod projects;
