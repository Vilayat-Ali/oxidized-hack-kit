pub mod db;
pub mod middlewares;
pub mod response;
pub mod routes;
pub mod utils;

pub type Ctx = Arc<Mutex<AppState>>;

use std::sync::Arc;

use crate::db::{user::JWTUserPayload, Mongo};
use envy::from_env;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct ENV {
    pub rust_log: String,
    pub port: u16,
    pub mongo_uri: String,
    pub jwt_secret: String,
}

impl ENV {
    pub fn import() -> ENV {
        dotenv::dotenv().ok();
        // We can `unwrap` here which makes our program to panic
        // once there is some irregularities or problem with our
        // environment variables.
        // Since, environment variables bugs and irregularities are
        // a serious issue, it is relatively safer to panic out from
        // the program rather safely handling it and keep program in
        // execution.
        match from_env::<ENV>() {
            Ok(env) => env,
            Err(e) => {
                tracing::error!("Invalid env variable configuration. {}", e);
                panic!("{}", e);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub mongo_instance: Mongo,
    pub user: Option<JWTUserPayload>,
}

impl AppState {
    pub fn new(mongo_instance: Mongo) -> Self {
        Self {
            mongo_instance,
            user: None,
        }
    }
}
