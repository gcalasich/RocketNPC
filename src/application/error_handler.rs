use crate::model::api_error;
use crate::model::api_response;
use chrono::Utc;
use log::error;
use rocket::serde::json::Json;

pub fn handle_error(
    error_title: String,
    error_description: String,
    api_error_type: api_error::APIErrorTypes,
    error_info: String,
) -> Json<api_response::ApiResponseError> {
    // const METHOD : &str = "handle_error";
    error!(target:"app::apierrors", "{} - {} - {}", error_title, error_description, error_info);
    let response: api_response::ApiResponseError = api_response::ApiResponseError::new(
        error_title.to_owned(),
        Utc::now(),
        String::from("API execution failed"),
        api_error_type as i32,
    );
    Json(response)
}

pub fn handle_error_struct(
    api_error_obj: api_error::APIError,
    friendly_description: &str,
    friendly_message: &str,
) -> Json<api_response::ApiResponseError> {
    error!(target:"app::apierrors", "{} - {} - {} - {} - {}", api_error_obj.error_code.to_string(), api_error_obj.error_message,  api_error_obj.file, api_error_obj.method, api_error_obj.error_info);

    let response: api_response::ApiResponseError = api_response::ApiResponseError::new(
        format!(
            "{} - {}",
            api_error_obj.error_code.to_string(),
            friendly_description.to_string()
        ),
        api_error_obj.error_datestamp,
        friendly_message.to_string(),
        api_error_obj.error_type as i32,
    );
    Json(response)
}
