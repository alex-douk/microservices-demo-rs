use tarpc::serde::Deserialize;
#[derive(Deserialize)]
#[serde(crate="tarpc::serde")]
pub(crate) struct Config {
    pub username: String,
    pub password: String,
    pub database: String,
    pub prime: bool,
}

#[derive(Deserialize)]
#[serde(crate="tarpc::serde")]
pub struct Data {
    db_config: Config,
}

impl Config {
    pub fn new() -> Self {
        let filename = "data/db_config.toml";
        let contents = match std::fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => panic!("Failed to read config"),
        };
        let data: Data = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => panic!("Failed to parse the config"),
        };
        data.db_config
    }
}
