pub struct AppConfig {
    pub database_url: String,
    pub whatever: String,
}

pub fn load_config() -> AppConfig {
    AppConfig {
        database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL to be available"),
        whatever: "whatever".to_string(),
    }
}