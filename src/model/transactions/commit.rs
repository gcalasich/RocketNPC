use rocket::serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub transactionId: i32,
    pub token: String,
}
