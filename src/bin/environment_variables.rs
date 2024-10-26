use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Postgres {
    pub user: String,
    pub password: String,
    pub database: String,
    pub port: String,
    pub host: String,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub postgres: Postgres,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize::<Settings>()?;

        return Ok(settings);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let settings = Settings::new();
    println!("{settings:#?}");

    Ok(())
}
