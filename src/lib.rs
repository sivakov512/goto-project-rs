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

    pub fn find(name: &str) -> Result<Config, String> {
        let mut path: PathBuf = env::home_dir().ok_or("Home dir not found on your system")?;
        path.push(name);

        let path_str = path.to_str().unwrap();
        match path.exists() {
            true => Ok(Config::new(path_str)),
            false => Err(format!("\"{}\" config not found", path_str)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{remove_file, File};

    #[test]
    fn find_returns_error_if_nothing_found() {
        let result = Config::find("nonexistence_config.yaml");

        let err = result.err().unwrap();
        assert!(err.contains("nonexistence_config.yaml\" config not found"));
    }

    #[test]
    fn find_returns_correct_config_if_found() {
        let _fake_config = FakeConfig::new(".test-config.yaml");

        let result = Config::find(".test-config.yaml");

        let config = result.unwrap();
        assert!(config.path.contains(".test-config.yaml"));
    }

    struct FakeConfig {
        path: String,
    }

    impl FakeConfig {
        fn new(name: &str) -> FakeConfig {
            let mut path: PathBuf = env::home_dir().unwrap();
            path.push(name);

            let path = path.to_str().unwrap();

            File::create(path).unwrap();

            FakeConfig {
                path: path.to_owned(),
            }
        }
    }

    impl Drop for FakeConfig {
        fn drop(&mut self) {
            let path = self.path.clone();
            remove_file(path).expect(
                format!("Something went wront when removing test file \"{}\"!", "a").as_str(),
            );
        }
    }
}
