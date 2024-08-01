use crate::models::politicians::Politician;
use diesel::{
    associations::{Associations, Identifiable},
    AsChangeset, Insertable, Queryable, Selectable,
};
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    Identifiable,
    Debug,
    Clone,
    AsChangeset,
    Insertable,
    Associations,
)]
#[diesel(belongs_to(Politician,foreign_key = politician_id))]
#[diesel(table_name=crate::models::schema::corruption_cases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CorruptionCase {
    pub id: i32,
    pub politician_id: i32,
    pub title: Option<String>,
    pub name: String,
    pub case_description: String,
    pub legal_outcome: Option<String>,
    pub case_date: String,
    pub downvotes: Option<i32>,
    pub upvotes: Option<i32>,
    pub link: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name=crate::models::schema::corruption_cases)]
pub struct NewCorruptionCase {
    pub politician_id: i32,
    pub name: Option<String>,
    pub title: Option<String>,
    pub case_description: Option<String>,
    pub legal_outcome: Option<String>,
    pub case_date: Option<String>,
    pub link: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
