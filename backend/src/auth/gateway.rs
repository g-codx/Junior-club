use actix_web::dev::ServiceRequest;
use crate::auth;


const CREATE_PATH: &str = "/create";
const EDIT_PATH: &str = "/edit";
const DELETE_PATH: &str = "/delete";
const GET_PATH: &str = "/get";
const PUBLISH_PATH: &str = "/publish";

fn is_privilege_request(req: &mut ServiceRequest) -> bool {
    (req.path().contains(CREATE_PATH) && !req.path().contains("/user"))
        || req.path().contains(EDIT_PATH)
        || req.path().contains(DELETE_PATH)
        || req.path().contains(PUBLISH_PATH)
}

fn is_get_request(req: &mut ServiceRequest) -> bool {
    req.path().contains(GET_PATH)
}

pub fn passed(mut req: &mut ServiceRequest) -> bool {
    log::info!("Gateway.. Request path:");

    if is_privilege_request(req) {
        log::info!("privilege request");

        match auth::process_token(&mut req) {
            Ok(token) => {
                log::info!("{}", token);
                log::info!("passed = true");

                true
            },
            Err(message) => {
                log::info!("passed = false. {}", message);

                false
            }
        }
    }
    else if is_get_request(req) {
        auth::process_user_role_token(&mut req)
    }
    else {
        log::info!("{}", req.path());
        true
    }
}


// const CREATE_POST_PATH: &str = "/post/create";
// const GET_POST_PATH: &str = "/post/get";
// const GET_LINK_PATH: &str = "/link";
//
//
// pub fn passed(mut req: &mut ServiceRequest) -> bool {
//     log::info!("Gateway.. Request path:");
//
//     if req.path().contains(CREATE_POST_PATH) {
//         log::info!("{}", CREATE_POST_PATH);
//         match auth::process_token(&mut req) {
//             Ok(token) => {
//                 log::info!("passed = true");
//                 true
//             },
//             Err(message) => {
//                 log::info!("passed = false. {}", message);
//                 false
//             }
//         }
//     }
//     else if req.path().contains(GET_POST_PATH) {
//         log::info!("{}", GET_POST_PATH);
//         auth::process_user_role_token(&mut req)
//     }
//     else if req.path().contains(GET_LINK_PATH) {
//         log::info!("{}", GET_LINK_PATH);
//         auth::process_user_role_token(&mut req)
//     }
//     else {
//         log::info!("{}", req.path());
//         true
//     }
// }


