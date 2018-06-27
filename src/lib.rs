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
    use std::fs::{remove_file, File};

    #[test]
    fn find_returns_error_if_nothing_found() {
        let result = Config::find("nonexistence_config.yaml");

        assert!(result.is_err())
    }

    #[test]
    fn find_returns_correct_config_if_found() {
        let _fake_config = FakeConfig::new(".test-config.yaml");

        let result = Config::find(".test-config.yaml");

        let config = result.unwrap();
        assert!(
            config
                .path
                .as_str()
                .contains(".test-config.yaml")
        );
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
