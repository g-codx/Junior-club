use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use crate::model::post::post::SingleQueryResult;
use crate::model::user::user::User;

const MOSCOW_TIME_OFFSET: i32 = 3 * 3600;
const DATE_FORMAT_PATTERN: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    // pub user_id: i32, ?
    pub title: String,
    pub preview: Option<String>,
    pub body: Option<String>,
    pub public_date: Option<String>,
    pub published: bool,
    // pub search_tag: String ?
    pub is_user_created: bool
}

impl Post {
    pub fn new_feed_item(model: (i32, String, String, bool, Option<NaiveDateTime>)) -> Self {
        Self {
            id: model.0,
            title: model.1,
            preview: Some(model.2),
            body: None,
            published: model.3,
            public_date: date_format(model.4),
            is_user_created: false
        }
    }

    pub fn new_post(model: SingleQueryResult, user: Option<User>) -> Self {
        let is_user_created = if let Some(user) = user {
            user.id == model.1 || user.role == "admin"
        } else {
            false
        };

        Self {
            id: model.0,
            title: model.2,
            preview: None,
            body: Some(model.3),
            published: model.4,
            public_date: date_format(model.5),
            is_user_created
        }
    }

    pub fn empty(message: String) -> Self {
        Self {
            id: 0,
            title: message,
            preview: None,
            body: None,
            published: false,
            public_date: None,
            is_user_created: false
        }
    }
}

impl Responder for Post {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self.body {
            None => HttpResponse::BadRequest().body("invalid param"),
            _ => {
                let res_body = serde_json::to_string(&self).unwrap();
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(res_body)
            }
        }
    }
}

fn date_format(date: Option<NaiveDateTime>) -> Option<String> {
    if let Some(date) = date {
        let tz_offset = FixedOffset::west(MOSCOW_TIME_OFFSET);

        let dt_with_tz: DateTime<FixedOffset> = tz_offset.from_local_datetime(&date).unwrap();
        let dt_with_tz_utc: DateTime<Utc> = Utc.from_utc_datetime(&dt_with_tz.naive_utc());

        Option::from(dt_with_tz_utc.format(DATE_FORMAT_PATTERN).to_string())
    } else {
        None
    }
}


#[derive(Serialize)]
pub struct Posts {
    pub posts: Vec<Post>,
    pub total_count: i64,
}

impl Posts {
    pub fn new(posts: Vec<(i32, String, String, bool, Option<NaiveDateTime>)>, total_count: i64) -> Self {
        let posts = posts
            .iter()
            .map(|p| Post::new_feed_item(p.clone()))
            .collect();

        Self {
            posts,
            total_count,
        }
    }
}

impl Responder for Posts {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}

#[derive(Deserialize)]
pub struct NewPostSchema {
    pub title: String,
    pub body: String,
    pub search_tag: String,
}

#[derive(Deserialize)]
pub struct GetQuery {
    pub tag: String,
    pub page: i64,
}

#[derive(Deserialize)]
pub struct PublishQuery {
    pub id: i32,
    pub publish: bool,
}

#[derive(Deserialize, Debug)]
pub struct EditQuery {
    pub id: i32,
    pub title: String,
    pub body: String,
}


