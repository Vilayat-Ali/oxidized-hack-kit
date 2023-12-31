pub mod db;
pub mod middlewares;
pub mod routes;
pub mod utils;

use envy::from_env;
use serde::{Deserialize, Serialize};

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
        from_env::<ENV>().unwrap()
    }
}
