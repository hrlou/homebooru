use clap::Parser;
// use serde::{Deserialize, Serialize};
// use std::sync::{Arc, RwLock};
use std::path::PathBuf;

lazy_static::lazy_static! {
	/// Configurations for this specific application server
	pub static ref SERVER: Server = Server::parse();
}

/// Configurations for this specific application server
#[derive(Parser, Debug)]
pub struct Server {
    /// Address for the server to listen on
	#[clap(short, long, default_value = "127.0.0.1:8080", env = "HOMEBOORU_ADDRESS")]
	pub address: String,

    /// Directory
    #[clap(short, long, default_value = "homebooru", env = "HOMEBOORU_PATH")]
    pub path: PathBuf,

    /// Database url
    #[clap(short, long, env = "DATABASE_URL")]
    pub database: Option<String>,

	/// Lowest log message level to output to stderr.
	// One of: ERROR WARN INFO DEBUG TRACE
	#[cfg(debug_assertions)]
	#[clap(short, long, env = "LOG_LEVEL", default_value = "WARN")]
	pub log_level: log::Level,
}

pub fn database() -> String {
    match &SERVER.database {
        Some(x) => x.to_string(),
        None => {
            format!("sqlite:{}", SERVER.path
                .join("db")
                .join("homebooru.db")
                .to_str()
                .unwrap()
            )
        },
    }
}