use actix_web::web;
use crate::model::post::post_criteria::Query::{AllByTag, Published, PublishedByTag, UserQuery, UserQueryByTag};
use crate::model::post::post_criteria::Tag::{All, Java, Rust};
use crate::model::user::user::User;
use crate::structs::post::GetQuery;


pub struct Criteria {
    pub page: i64,
    pub tag: Option<Tag>,
    pub user_id: Option<i32>,
    pub query: Query,
}

impl Criteria {
    pub fn from(req_query: web::Query<GetQuery>, user: Option<User>) -> Self {
        let page = Criteria::get_offset(req_query.page);
        let tag = Tag::tag(req_query.tag.clone());

        match user {
            Some(usr) => {
                if Criteria::is_admin(&usr) {
                    let query = if tag.with_tag() { AllByTag } else { Query::All };
                    Criteria::new(page, Some(tag), None, query)
                } else {
                    let query = if tag.with_tag() { UserQueryByTag } else { UserQuery };
                    Criteria::new(page, Some(tag), Some(usr.id.clone()), query)
                }
            }
            None => {
                let query = if tag.with_tag() { PublishedByTag } else { Published };
                Criteria::new(page, Some(tag), None, query)
            }
        }
    }

    fn new(page: i64, tag: Option<Tag>, user_id: Option<i32>, query: Query) -> Criteria {
        Self {
            page,
            tag,
            user_id,
            query,
        }
    }


    fn get_offset(query_page: i64) -> i64 {
        if query_page < 0 {
            return 0;
        }
        match query_page {
            1 | 0 => 0,
            2 => 10,
            _ => (query_page - 1) * 10
        }
    }

    fn is_admin(user: &User) -> bool {
        user.role.to_lowercase() == "admin".to_string()
    }
}


pub enum Query {
    All,
    AllByTag,
    Published,
    PublishedByTag,
    UserQuery,
    UserQueryByTag,
}

#[derive(PartialEq)]
pub enum Tag {
    All,
    Java(String),
    Rust(String),
}

impl Tag {
    pub fn tag(req_tag: String) -> Self {
        match req_tag.to_lowercase().as_str() {
            "java" => Java("Java".to_string()),
            "rust" => Rust("Rust".to_string()),
            _ => All
        }
    }

    pub fn with_tag(&self) -> bool {
        !(&All == self)
    }

    pub fn get_tag(&self) -> String {
        match self {
            Java(val) => val.to_string(),
            Rust(val) => val.to_string(),
            _ => "".to_string()
        }
    }
}
