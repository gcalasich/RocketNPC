use chrono::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub userName: String,
    pub userPassword: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Authentication {
    pub user_id: i32,
    pub role_id: i32,
    pub last_authentication: DateTime<Utc>,
    pub exp: usize,
}

impl Authentication {
    pub fn new(user_id: i32, role_id: i32, last_authentication: DateTime<Utc>) -> Authentication {
        Authentication {
            user_id,
            role_id,
            last_authentication,
            exp: 0,
        }
    }
}

impl fmt::Display for Authentication {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "user_id = {} - role_id = {} - last_authentication = {}",
            self.user_id, self.role_id, self.last_authentication
        )
    }
}
