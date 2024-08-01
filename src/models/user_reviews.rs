use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::models::schema::user_reviews)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserReview {
    pub id: i32,
    pub case_id: i32,
    pub title: String,
    pub review_text: String,
    pub downvotes: Option<i32>,
    pub upvotes: Option<i32>,
    pub link: Option<String>,
    pub user_id: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate, Serialize, Clone, Insertable)]
#[diesel(table_name=crate::models::schema::user_reviews)]
pub struct NewUserReview {
    pub case_id: i32,
    pub title: String,
    pub link: Option<String>,
    pub review_text: String,
    pub user_id: Option<String>,
}
