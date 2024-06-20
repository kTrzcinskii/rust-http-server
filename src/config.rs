use std::env;

pub struct Config {
    files_directory: String,
}

const FILES_DIRECTORY_ARG_NAME: &str = "--directory";

impl Config {
    pub fn load() -> Self {
        let mut directory: String = String::from(".");
        let args: Vec<String> = env::args().collect();
        if let Some(index) = args
            .iter()
            .position(|arg_name| arg_name == FILES_DIRECTORY_ARG_NAME)
        {
            if let Some(arg_value) = args.get(index + 1) {
                directory = arg_value.clone();
            }
        }
        Config {
            files_directory: directory,
        }
    }

    pub fn get_files_directory(&self) -> &str {
        &self.files_directory
    }
}
