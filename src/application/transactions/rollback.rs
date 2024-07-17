use crate::model::api_error::APIError;
use crate::model::api_response;
use crate::model::transactions::rollback;
use rocket::serde::json::Json;

//const FILE: &str = "application/rollback.rs";

pub fn rollback(
    rollback: &rollback::Rollback,
) -> Result<Json<api_response::ApiResponse>, APIError> {
    let _rollback_transaction = rollback_transaction(rollback)?;

    let response: api_response::ApiResponse = api_response::ApiResponse::new(
        String::from(""),
        String::from(""),
        String::from("Successful Rollback performed"),
        0,
    );
    Ok(Json(response))
}

pub fn rollback_transaction(rollback: &rollback::Rollback) -> Result<bool, APIError> {
    let _transaction_id = rollback.transactionId;
    Ok(true)
}
