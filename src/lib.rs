use std::env;
use std::path::PathBuf;

pub struct Config {
    path: String,
}

impl Config {
    fn new(path: &str) -> Config {
        let path = path.to_owned();
        Config { path }
    }

    pub fn find(name: &str) -> Result<Config, &'static str> {
        let mut path: PathBuf = env::home_dir().ok_or("Home dir not found on your system")?;
        path.push(name);

        match path.exists() {
            true => Ok(Config::new(path.to_str().unwrap())),
            false => Err("Zhopa"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_returns_error_if_nothing_found() {
        let result = Config::find("nonexistence_config.yaml");

        assert!(result.is_err())
    }

    #[test]
    fn find_returns_correct_config_if_found() {
        let result = Config::find(".goto-project.yaml");

        let config = result.unwrap();
        assert_eq!(
            config.path,
            "/home/sivakov512/.goto-project.yaml".to_owned()
        );
    }
}
