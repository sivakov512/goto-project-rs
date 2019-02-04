use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct Config {
    path: String,
}

impl Config {
    pub fn new(name: &str) -> Config {
        let mut path: PathBuf = dirs::home_dir().unwrap();
        path.push(name);

        let path_str = path.to_str().unwrap();

        if !path.exists() {
            panic!(format!("\"{}\" config not found", path_str))
        }

        Config {
            path: path_str.to_owned(),
        }
    }
}

pub trait ConfigLoader {
    fn load(&self) -> String;
}

impl ConfigLoader for Config {
    fn load(&self) -> String {
        let mut file = File::open(&self.path).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        contents
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use std::io::Write;

    #[test]
    fn new_returns_correct_config() {
        let fake_config = FakeConfig::new("conf1.yaml", "");

        let config = Config::new("conf1.yaml");

        assert_eq!(&config.path, &fake_config.path);
    }

    #[test]
    #[should_panic]
    fn new_should_panic_if_config_file_not_found() {
        Config::new("nonexistence_config.yaml");
    }

    #[test]
    fn load_returns_config_content() {
        let _fake_config = FakeConfig::new("conf2.yaml", "Awesome content");
        let config = Config::new("conf2.yaml");

        assert_eq!(config.load(), "Awesome content");
    }

    struct FakeConfig {
        path: String,
    }

    impl FakeConfig {
        fn new(name: &str, contents: &str) -> FakeConfig {
            let mut path: PathBuf = dirs::home_dir().unwrap();
            path.push(name);

            let path = path.to_str().unwrap();
            let mut file = File::create(path).unwrap();

            file.write_all(contents.as_bytes()).unwrap();
            file.flush().unwrap();

            FakeConfig {
                path: path.to_owned(),
            }
        }
    }

    impl Drop for FakeConfig {
        fn drop(&mut self) {
            let path = self.path.clone();
            remove_file(path).unwrap_or_else(|_| {
                panic!(format!(
                    "Something went wront when removing test file \"{}\"!",
                    "a"
                ))
            });
        }
    }
}
