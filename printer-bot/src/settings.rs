use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use telegram_bot::UserId;
use tracing::{info, warn};

use std::{
    fs::{self, File},
    io::Write,
    marker::PhantomData,
};

use super::Error;

const SETTINGS_PATH: &str = "./settings.toml";

lazy_static! {
    pub static ref SETTINGS: Settings =
        Settings::load_or_create_default().expect("Failed to open settings");
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub max_print_len: usize,
    pub minutes_between_prints: usize,
    #[serde(default, skip_serializing)]
    _cannot_create: PhantomData<()>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub role: String,
    #[serde(default, skip_serializing)]
    _cannot_create: PhantomData<()>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub roles: Vec<Role>,
    pub users: Vec<User>,
    #[serde(default, skip_serializing)]
    _cannot_create: PhantomData<()>,
}

impl Settings {
    pub fn get_role(&self, id: UserId) -> Option<&Role> {
        self.get_user(id).and_then(|user| {
            let role_name = &user.role;
            self.roles.iter().find(|role| role.name == *role_name)
        })
    }

    pub fn get_user(&self, id: UserId) -> Option<&User> {
        self.users.iter().find(|user| {
            let user_id = user.id.parse().expect("User ID not an int");
            id == UserId::new(user_id)
        })
    }

    fn load_or_create_default() -> Result<Self, Error> {
        match fs::read_to_string(SETTINGS_PATH).map_err(Error::OpeningSettingsFile) {
            Ok(content) => toml::from_str(&content).map_err(Error::ParsingSettingsFile),
            Err(why) => {
                warn!("{}", why);
                info!("creating default settings");
                let settings = Settings {
                    roles: vec![
                        Role {
                            name: String::from("admin"),
                            max_print_len: usize::MAX,
                            minutes_between_prints: 0,
                            _cannot_create: PhantomData,
                        },
                        Role {
                            name: String::from("user"),
                            max_print_len: 200,
                            minutes_between_prints: 3600,
                            _cannot_create: PhantomData,
                        },
                    ],
                    users: vec![User {
                        id: String::from("228223333"),
                        role: String::from("admin"),
                        _cannot_create: PhantomData,
                    }],
                    _cannot_create: PhantomData,
                };
                let content =
                    toml::to_string_pretty(&settings).expect("BUG: Invalid default config");
                let mut file = File::create(SETTINGS_PATH).map_err(Error::CreatingSettingsFile)?;
                write!(file, "{}", content).map_err(Error::CreatingSettingsFile)?;
                Ok(settings)
            }
        }
    }
}