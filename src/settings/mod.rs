use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Postgres {
    pub username: String,
    pub password: String,
    pub database: String,
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub postgres: Postgres,
}

impl Settings {
    pub fn new() -> Self {
        let settings = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()
            .unwrap()
            .try_deserialize::<Settings>()
            .unwrap();

        settings
    }
}
