use crate::{
    db::user::{Role, UserName},
    utils::jwt::JWT,
};
use axum::extract::Json;
use serde::{Deserialize, Serialize};

use crate::db::user::{JWTUserPayload, User};
