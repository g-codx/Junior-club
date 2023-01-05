use diesel::prelude::*;
use crate::schema::posts;
use super::super::user::user::User;
use chrono::NaiveDateTime;

use crate::schema::posts::dsl::*;

type SelectType = (i32, String, String, bool, Option<NaiveDateTime>);
pub type VecQueryResult = (Vec<SelectType>, i64);
pub type SingleQueryResult = (i32, i32, String, String, bool, Option<NaiveDateTime>);

#[belongs_to(User)]
#[table_name = "posts"]
#[derive(Queryable, Identifiable, Associations, Debug)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub preview: String,
    pub body: String,
    pub published: bool,
    pub public_date: Option<NaiveDateTime>,
    pub search_tag: String,
}

impl Post {
    pub fn find_by_id(post_id: i32, connection: &mut PgConnection) -> SingleQueryResult {
        posts
            .select((id, user_id, title, body, published, public_date))
            .filter(id.eq(post_id))
            .first::<(i32, i32, String, String, bool, Option<NaiveDateTime>)>(connection)
            .unwrap()
    }

    pub fn find_all(offset: i64, connection: &mut PgConnection) -> VecQueryResult {
        let result = posts
            .select((id, title, preview, published, public_date))
            .limit(10)
            .offset(offset)
            .load::<(i32, String, String, bool, Option<NaiveDateTime>)>(connection)
            .unwrap();
        let total = posts
            .select((id, title, preview, published, public_date))
            .count()
            .get_result::<i64>(connection)
            .unwrap();

        (result, total)
    }

    pub fn find_all_by_tag(offset: i64, tag: String, connection: &mut PgConnection) -> VecQueryResult {
        let result = posts
            .select((id, title, preview, published, public_date))
            .filter(search_tag.eq(tag.clone()))
            .limit(10)
            .offset(offset)
            .load::<(i32, String, String, bool, Option<NaiveDateTime>)>(connection)
            .unwrap();
        let total = posts
            .select((id, title, preview, published, public_date))
            .filter(search_tag.eq(tag.clone()))
            .count()
            .get_result::<i64>(connection)
            .unwrap();

        (result, total)
    }

    pub fn find_published(offset: i64, connection: &mut PgConnection) -> VecQueryResult {
        let result = posts
            .select((id, title, preview, published, public_date))
            .filter(published.eq(true))
            .limit(10)
            .offset(offset)
            .load::<(i32, String, String, bool, Option<NaiveDateTime>)>(connection)
            .unwrap();
        let total = posts
            .select((id, title, preview, published, public_date))
            .filter(published.eq(true))
            .count()
            .get_result::<i64>(connection)
            .unwrap();

        (result, total)
    }

    pub fn find_published_by_tag(offset: i64, tag: String, connection: &mut PgConnection) -> VecQueryResult {
        let result = posts
            .select((id, title, preview, published, public_date))
            .filter(published.eq(true))
            .filter(search_tag.eq(tag.clone()))
            .limit(10)
            .offset(offset)
            .load::<(i32, String, String, bool, Option<NaiveDateTime>)>(connection)
            .unwrap();
        let total = posts
            .select((id, title, preview, published, public_date))
            .filter(published.eq(true))
            .filter(search_tag.eq(tag.clone()))
            .count()
            .get_result::<i64>(connection)
            .unwrap();

        (result, total)
    }

    pub fn find_published_or_user_created(offset: i64, usr_id: i32, connection: &mut PgConnection) -> VecQueryResult {
        let result = posts
            .select((id, title, preview, published, public_date))
            .filter(published.eq(true))
            .or_filter(user_id.eq(usr_id))
            .limit(10)
            .offset(offset)
            .load::<(i32, String, String, bool, Option<NaiveDateTime>)>(connection)
            .unwrap();
        let total = posts
            .select((id, title, preview, published, public_date))
            .filter(published.eq(true))
            .or_filter(user_id.eq(usr_id))
            .count()
            .get_result::<i64>(connection)
            .unwrap();

        (result, total)
    }

    pub fn find_published_or_user_created_by_tag(offset: i64, usr_id: i32, tag: String, connection: &mut PgConnection) -> VecQueryResult {
        let result = posts
            .select((id, title, preview, published, public_date))
            .filter(published.eq(true))
            .filter(search_tag.eq(tag.clone()))
            .or_filter(user_id.eq(usr_id))
            .limit(10)
            .offset(offset)
            .load::<(i32, String, String, bool, Option<NaiveDateTime>)>(connection)
            .unwrap();
        let total = posts
            .select((id, title, preview, published, public_date))
            .filter(published.eq(true))
            .filter(search_tag.eq(tag.clone()))
            .or_filter(user_id.eq(usr_id))
            .count()
            .get_result::<i64>(connection)
            .unwrap();

        (result, total)
    }
}