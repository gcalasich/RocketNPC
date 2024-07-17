use rocket::{
    data::{self, Data, FromData, ToByteUnit},
    http::{ContentType, Status},
    post,
    request::{self, Request},
    routes,
    serde::json::Json,
    Shutdown, State,
};

mod model;
use model::*;
mod application;
use application::*;
mod infrastructure;

use infrastructure::capnp_client;

use serde_json::Value;
use urlencoding::decode;

extern crate rocket;
const FILE: &str = "main.rs";

#[derive(Debug)]
pub enum RequestError {
    TooLarge,
    NoColon,
    Io(std::io::Error),
}

pub mod hello_world_capnp {
    include!(concat!(env!("OUT_DIR"), "\\src\\hello_world_capnp.rs"));
}

#[rocket::async_trait]
impl<'r> FromData<'r> for api_request::APIRequest<'r> {
    type Error = RequestError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::outcome::Outcome::*;
        use RequestError::*;

        // Ensure the content type is correct before opening the data.
        let request_api_ct = ContentType::new("application", "x-www-form-urlencoded");
        if req.content_type() != Some(&request_api_ct) {
            return Forward((data, Status::Ok));
        }

        // Use a configured limit with name 'person' or fallback to default.
        let limit = req.limits().get("persona").unwrap_or(4048.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Error((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Error((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);

        println!("api - {}", string);

        // Split the string into two pieces at ':'.
        let (api_name, api_params) = match string.find('&') {
            Some(i) => (&string[..i], &string[(i + 1)..]),
            None => return Error((Status::UnprocessableEntity, NoColon)),
        };

        let api_name = match api_name.find('=') {
            Some(i) => &api_name[(i + 1)..],
            None => return Error((Status::UnprocessableEntity, NoColon)),
        };

        let api_params = match api_params.find('=') {
            Some(i) => &api_params[(i + 1)..],
            None => return Error((Status::UnprocessableEntity, NoColon)),
        };

        Success(api_request::APIRequest::new(api_name, api_params))
    }
}

#[post("/Shutdown", data = "<request>")]
fn shutdown(shutdown: Shutdown, request: api_request::APIRequest<'_>) -> &'static str {
    //const METHOD : &str = "Shutdown";
    println!("Shutting Down...");
    //let api_param_json = api_param_json_result.unwrap();
    //let api_name = &request.api_name.to_string().to_uppercase();
    shutdown.notify();
    "Shutting down..."
}

#[post("/InvokeAPI", data = "<request>")]
async fn invoke(
    rpc_client: &State<capnp_client::SerializingRpcClient>,
    request: api_request::APIRequest<'_>,
) -> Result<Json<api_response::ApiResponse>, Json<api_response::ApiResponseError>> {
    const METHOD: &str = "invoke";
    println!("Invoke");
    let api_param = &request.api_params.replace('+', " ");
    let api_param_json_result = decode(api_param);
    match api_param_json_result {
        Ok(ref v) => v,
        Err(e) => {
            return Err(error_handler::handle_error(
                format!(
                    "{} - {}",
                    api_error::APIErrorCodes::MAIINV01,
                    String::from("Error Decoding API PARAM JSON")
                ),
                format!("Error: {} - File: {} - Method: {}", e, FILE, METHOD),
                api_error::APIErrorTypes::GeneralException,
                request.api_params.to_string(),
            ))
        }
    };
    let api_param_json = api_param_json_result.unwrap();
    let api_name = &request.api_name.to_string().to_uppercase();
    let _result_ser = serde_json::from_str(&api_param_json);
    if let Err(e) = _result_ser {
        return Err(error_handler::handle_error(
            format!(
                "{} - {}",
                api_error::APIErrorCodes::MAIINV04,
                String::from("Error Deserializing API Request JSON")
            ),
            format!("Error: {} - File: {} - Method: {}", e, FILE, METHOD),
            model::api_error::APIErrorTypes::GeneralException,
            format!("{} - {}", request.api_name, request.api_params),
        ));
    }
    let _json_values: Value = _result_ser.unwrap();

    if api_name != "AUTHENTICATE" {
        //#TODO - Authenticate Perform Validation
    }

    match api_name as &str {
        "AUTHENTICATE" => {
            let result_ser = serde_json::from_str(&api_param_json);
            if let Err(e) = result_ser {
                return Err(error_handler::handle_error(
                    format!(
                        "{} - {}",
                        api_error::APIErrorCodes::MAIINV03,
                        String::from("Error Deserializing API Request JSON")
                    ),
                    format!("Error: {} - File: {} - Method: {}", e, FILE, METHOD),
                    model::api_error::APIErrorTypes::GeneralException,
                    request.api_params.to_string(),
                ));
            }
            let userlogin: model::authenticate::Login = result_ser.unwrap();
            let authenticate_result = application::authenticate::authenticate(userlogin);
            if let Err(e) = authenticate_result {
                return Err(error_handler::handle_error_struct(
                    e,
                    &String::from("Authentication Error"),
                    &String::from("API Execution failed."),
                ));
            }
            let authenticate = authenticate_result.unwrap();
            Ok(authenticate)
        }
        "STARTTRANSACTION" => {
            let result_ser = serde_json::from_str(&api_param_json);

            if let Err(e) = result_ser {
                return Err(error_handler::handle_error(
                    format!(
                        "{} - {}",
                        api_error::APIErrorCodes::MAIINV06,
                        String::from("Error Deserializing API Request JSON")
                    ),
                    format!("Error: {} - File: {} - Method: {}", e, FILE, METHOD),
                    model::api_error::APIErrorTypes::GeneralException,
                    request.api_params.to_string(),
                ));
            }
            let start_transaction: model::transactions::start_transaction::StartTransaction =
                result_ser.unwrap();
            validate_token(&request.api_params.to_owned(), &start_transaction.token)?;
            let response_result =
                application::transactions::start_transaction::start_transaction(&start_transaction);
            if let Err(e) = response_result {
                return Err(error_handler::handle_error_struct(
                    e,
                    &String::from("Start Transaction failed"),
                    &String::from("API Execution failed."),
                ));
            }

            Ok(response_result.unwrap())
        }
        "COMMIT" => {
            let result_ser = serde_json::from_str(&api_param_json);
            if let Err(e) = result_ser {
                return Err(error_handler::handle_error(
                    format!(
                        "{} - {}",
                        api_error::APIErrorCodes::MAIINV07,
                        String::from("Error Deserializing API Request JSON")
                    ),
                    format!("Error: {} - File: {} - Method: {}", e, FILE, METHOD),
                    model::api_error::APIErrorTypes::GeneralException,
                    request.api_params.to_string(),
                ));
            }
            let commit: model::transactions::commit::Commit = result_ser.unwrap();

            let response_result = application::transactions::commit::commit(&commit);
            if let Err(e) = response_result {
                return Err(error_handler::handle_error_struct(
                    e,
                    &String::from("Commit Transaction failed"),
                    &String::from("API Execution failed."),
                ));
            }
            Ok(response_result.unwrap())
        }
        "ROLLBACK" => {
            let result_ser = serde_json::from_str(&api_param_json);
            if let Err(e) = result_ser {
                return Err(error_handler::handle_error(
                    format!(
                        "{} - {}",
                        api_error::APIErrorCodes::MAIINV08,
                        String::from("Error Deserializing API Request JSON")
                    ),
                    format!("Error: {} - File: {} - Method: {}", e, FILE, METHOD),
                    model::api_error::APIErrorTypes::GeneralException,
                    request.api_params.to_string(),
                ));
            }
            let rollback: model::transactions::rollback::Rollback = result_ser.unwrap();

            let response_result = application::transactions::rollback::rollback(&rollback);
            if let Err(e) = response_result {
                return Err(error_handler::handle_error_struct(
                    e,
                    &String::from("Rollback Transaction failed"),
                    &String::from("API Execution failed."),
                ));
            }
            Ok(response_result.unwrap())
        }

        "TEST" => {
            let hello = rpc_client
                .say_hello_request(String::from("Hi, capnp!"))
                .await;
            println!("Got: {}", hello);

            Err(error_handler::handle_error(
                String::from("entro"),
                String::from("Entro"),
                model::api_error::APIErrorTypes::APINotImplemented,
                String::from("API not implemented"),
            ))
        }
        _ => Err(error_handler::handle_error(
            String::from("API not implemented"),
            String::from("API not implemented"),
            model::api_error::APIErrorTypes::APINotImplemented,
            String::from("API not implemented"),
        )),
    }
}

fn validate_token(
    api_params: &String,
    token: &String,
) -> Result<(), Json<api_response::ApiResponseError>> {
    const METHOD: &str = "validate_token";
    let validate_token_result = application::validate_token::is_token_valid(token);
    if let Err(e) = validate_token_result {
        return Err(error_handler::handle_error_struct(
            e,
            &String::from("Failed Validating Token"),
            &String::from("API Execution failed."),
        ));
    }
    let validate_token = validate_token_result.unwrap();
    if !validate_token {
        return Err(error_handler::handle_error(
            format!(
                "{} - {}",
                api_error::APIErrorCodes::MAIVTO02,
                String::from("Token Expired")
            ),
            format!(
                "Error: {} - File: {} - Method: {}",
                "Token Expired", FILE, METHOD
            ),
            model::api_error::APIErrorTypes::GeneralException,
            api_params.to_string(),
        ));
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    log4rs::init_file("./src/log4rs.yaml", Default::default()).unwrap();

    // setup RPC
    let client = capnp_client::SerializingRpcClient::new("127.0.0.1:4000").await;

    rocket::build()
        .mount("/ExternalAPI", routes![invoke])
        .mount("/Admin", routes![shutdown])
        .manage(client)
        .launch()
        .await
        .ok();
}
