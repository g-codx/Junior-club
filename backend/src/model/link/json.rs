use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};

type Model = crate::model::link::link::LinkInfo;

#[derive(Deserialize)]
pub struct GetQuery {
    pub section: String
}

#[derive(Deserialize)]
pub struct EditQuery {
    pub id: i32,
    pub title: String,
    pub link: String,
}

#[derive(Serialize)]
pub struct LinksResponse {
    pub links: Vec<Model>,
    pub is_admin: bool
}

impl LinksResponse {
    pub fn json_response(links: Vec<Model>, is_admin: bool) -> Self {
        Self { links, is_admin }
    }
}

impl Responder for LinksResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}

