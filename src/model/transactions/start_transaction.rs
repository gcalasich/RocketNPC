use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StartTransaction{
    pub category: String, 
    pub name: String, 
    pub readOnly: bool, 
    pub token: String
}