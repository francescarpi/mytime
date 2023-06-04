use ini::Ini;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Debug, Clone)]
pub enum DbType {
    Sqlite,
}

#[derive(Debug, Clone)]
pub enum IntegrationType {
    Redmine,
}


#[derive(Debug, Clone)]
pub struct Config {
    pub app_share_path: PathBuf,
    pub redmine_url: Option<String>,
    pub redmine_token: Option<String>,
    pub db_type: DbType,
    pub int_type: IntegrationType,
}

#[derive(Debug)]
struct ConfigFile {
    pub app_share_path: Option<PathBuf>,
    pub redmine_url: Option<String>,
    pub redmine_token: Option<String>,
    pub db_type: Option<DbType>,
    pub int_type: Option<IntegrationType>,
}

impl Config {
    pub fn new() -> Self {
        let home = env::var("HOME").unwrap();
        let config_from_ini_file = Self::load_config_file(&home);
        let default_config = Self::default_config(&home);
        let final_config = Self::build_final_config(&config_from_ini_file, &default_config);

        Self::create_share_folder_if_not_exist(&final_config.app_share_path.to_str().unwrap());

        Self {
            ..final_config
        }
    }

    fn load_config_file(home_path: &str) -> ConfigFile {
        // init file path: $HOME/.mytime
        let ini_file_path = Path::new(&home_path).join(".mytime");
        let mut ini_file_props: HashMap<String, &str> = HashMap::new();

        match Ini::load_from_file(&ini_file_path) {
            Ok(conf) => {
                for (sec, prop) in &conf {
                    for (key, value) in prop.iter() {
                        ini_file_props
                            .entry(format!("{}_{}", sec.unwrap(), key).to_string())
                            .or_insert(value);
                    }
                }

                let app_share_path = ini_file_props
                    .get("general_db_folder")
                    .map(|value| Path::new(&value).to_path_buf());
                let redmine_url = ini_file_props
                    .get("redmine_url")
                    .map(|value| value.to_string());
                let redmine_token = ini_file_props
                    .get("redmine_token")
                    .map(|value| value.to_string());
                let db_type = ini_file_props
                    .get("general_db")
                    .map(|value| DbType::from_string(value))
                    .flatten();
                let int_type = ini_file_props
                    .get("general_integration")
                    .map(|value| IntegrationType::from_string(value))
                    .flatten();

                ConfigFile {
                    app_share_path,
                    redmine_url,
                    redmine_token,
                    db_type,
                    int_type,
                }
            }
            Err(_) => ConfigFile {
                app_share_path: None,
                redmine_url: None,
                redmine_token: None,
                db_type: None,
                int_type: None,
            },
        }
    }

    fn default_config(home: &str) -> Config {
        let app_share_path = Path::new(&home).join(".local").join("share").join("mytime");
        Config {
            app_share_path,
            redmine_url: None,
            redmine_token: None,
            db_type: DbType::Sqlite,
            int_type: IntegrationType::Redmine,
        }
    }

    fn build_final_config(from_ini: &ConfigFile, default: &Config) -> Config {
        Config {
            app_share_path: from_ini
                .app_share_path
                .clone()
                .unwrap_or(default.app_share_path.clone()),
            redmine_url: from_ini.redmine_url.clone().or(default.redmine_url.clone()),
            redmine_token: from_ini.redmine_token.clone().or(default.redmine_token.clone()),
            db_type: from_ini.db_type.clone().unwrap_or(default.db_type.clone()),
            int_type: from_ini.int_type.clone().unwrap_or(default.int_type.clone()),
        }
    }

    fn create_share_folder_if_not_exist(share_path: &str) {
        if fs::metadata(&share_path).is_err() {
            fs::create_dir_all(&share_path)
                .expect("'mytime' configuration folder has not been created");
        }
    }
}

impl DbType {
    pub fn from_string(value: &str) -> Option<DbType> {
        match value.to_lowercase().as_str() {
            "sqlite" => Some(DbType::Sqlite),
            _ => None,
        }
    }
}

impl IntegrationType {
    pub fn from_string(value: &str) -> Option<IntegrationType> {
        match value.to_lowercase().as_str() {
            "redmine" => Some(IntegrationType::Redmine),
            _ => None,
        }
    }
}
