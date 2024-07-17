use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct APIRequest<'r> {
  pub  api_name: &'r str,
  pub  api_params: &'r str
}

impl <'r> APIRequest<'_> {
    pub fn new(api_name: &'r str, api_params:&'r str) -> APIRequest<'r>{
        APIRequest {api_name, api_params}
    }
 }