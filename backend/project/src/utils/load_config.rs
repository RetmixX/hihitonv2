use std::env;
use dotenv::dotenv;

pub struct ConfigApp {
    pub db: ConfigDB,
}

pub struct ConfigDB {
    pub db_url: String,
    pub max_connection: u32,
}

impl ConfigApp {
    pub fn new() -> Self {
        let (db_url, max_connection) = read_db_config();
        Self {
            db: ConfigDB { db_url, max_connection }
        }
    }

    pub fn load_db() -> Self {
        let (db_url, max_connection) = read_db_config();
        Self {
            db: ConfigDB { db_url, max_connection }
        }
    }
}


fn read_db_config() -> (String, u32) {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB URL NOT SET");
    let max_connection = env::var("MAX_CONNECTION")
        .expect("MAX_CONNECTION not set").parse::<u32>()
        .expect("Value in MAX_CONNECTION not a unsigned integer");

    (db_url, max_connection)
}