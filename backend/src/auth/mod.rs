use actix_web::dev::ServiceRequest;
use actix_web::http::header::HeaderValue;
pub mod jwt;
pub mod processes;
pub mod gateway;


/// Processes the token to see if the correct token is in the header.
///
/// # Parameters
/// * request (&ServiceRequest): the request passed through the view function
///
/// # Returns
/// * (Result<String, &'templates str>): processed token if successful, error message if not
pub fn process_token(request: &mut ServiceRequest) -> Result<String, &'static str> {
    match processes::extract_header_token(request) {
        Ok(token) => {
            match processes::check_password_and_get_user_id(token) {
                Ok(token) => {
                    request.head_mut().headers.insert("user_id".parse().unwrap(), HeaderValue::from(token));
                    Ok("passed".to_string())
                },
                Err(message) => Err(message)
            }
        },
        Err(message) => Err(message)
    }
}

/// Processes the token to see if the correct token is in the header.
/// Add user_id in header.
///
/// # Parameters
/// * request (&ServiceRequest): the request passed through the view function
///

pub fn process_user_role_token(request: &mut ServiceRequest) -> bool {
    match processes::extract_some_header_token(&request) {
        Some(token) => {
            match processes::check_password_and_get_user_id(token) {
                Ok(token) => {
                    request.head_mut().headers.insert("user_id".parse().unwrap(), HeaderValue::from(token));
                    true
                },
                Err(message) => {
                    println!("{}", message);
                    true
                }
            }
        },
        None => true
    }
}

