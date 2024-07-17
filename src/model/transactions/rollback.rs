use rocket::serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Rollback{
    pub transactionId: i32, 
    pub token: String
}

impl Rollback {
    pub fn new(transaction_id:i32, token: String) -> Rollback{
        Rollback {transactionId: transaction_id, token} 
    }
 }
