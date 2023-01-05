use std::num::ParseIntError;
use actix_web::HttpRequest;
use log::info;

pub fn parse_id_from_param(req: &HttpRequest) -> Result<i32, ParseIntError> {
    req.match_info().get("id").unwrap().parse::<i32>()
}

pub fn parse_user_id_from_header(req: &HttpRequest) -> Option<i32> {
    let header_value = req.head().headers.get("user_id");
    let string_user_id = match header_value {
        Some(str_id) => {
            match str_id.to_str() {
                Ok(str_id) => str_id.to_string(),
                Err(e) => {
                    info!("{}", e);
                    return None
                }
            }
        },
        None => {
            info!("user id not found");
            return None
        }
    };
    match string_user_id.parse::<i32>()  {
        Ok(id) => Some(id),
        Err(e) => {
            info!("{:?}", e);
            return None
        }
    }
}
