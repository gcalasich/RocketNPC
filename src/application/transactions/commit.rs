use crate::model::api_error::APIError;
use crate::model::api_response;
use crate::model::transactions::commit;
use rocket::serde::json::Json;

//const FILE: &str = "application/commit.rs";

pub fn commit(commit: &commit::Commit) -> Result<Json<api_response::ApiResponse>, APIError> {
    //const METHOD : &str = "commit";

    let _transaction_id = commit.transactionId;

    let response: api_response::ApiResponse = api_response::ApiResponse::new(
        String::from(""),
        String::from(""),
        String::from("Successful commit performed"),
        0,
    );
    Ok(Json(response))
}
