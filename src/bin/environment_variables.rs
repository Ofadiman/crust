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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let settings = config::Config::builder()
        .add_source(config::Environment::default().separator("__"))
        .build()?
        .try_deserialize::<Settings>()?;

    println!("{:#?}", settings);

    Ok(())
}
