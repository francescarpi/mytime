use ini::ini;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Debug, Clone)]
pub struct Config {
    pub app_share_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let app_share_path = Self::detect_share_path();
        let app_share_path_str = app_share_path.to_str().unwrap().to_string();

        Self::create_share_folder_if_not_exist(app_share_path_str.clone());

        Self { app_share_path }
    }

    fn detect_share_path() -> PathBuf {
        let home = env::var("HOME").unwrap();
        let ini_file_path = Path::new(&home).join(".mytime");
        let ini_file = ini!(safe ini_file_path.to_str().unwrap());

        match ini_file {
            Ok(ini_file) => {
                let share_path = String::from(ini_file["general"]["db_folder"].clone().unwrap());
                Path::new(&share_path).to_path_buf()
            }
            Err(_) => Path::new(&home).join(".local").join("share").join("mytime"),
        }
    }

    fn create_share_folder_if_not_exist(share_path: String) {
        if fs::metadata(&share_path).is_err() {
            fs::create_dir_all(&share_path)
                .expect("'mytime' configuration folder has not been created");
        }
    }
}
