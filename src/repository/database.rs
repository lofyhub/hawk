use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::cases::{CorruptionCase, NewCorruptionCase};
use crate::models::politicians::{NewPolitician, Politician};
use crate::models::user_reviews::{NewUserReview, UserReview};

use crate::models::schema::corruption_cases::dsl::{
    corruption_cases, created_at as cor_created_at, downvotes as case_downvotes, id as cor_case_id,
    politician_id as cor_id, upvotes as case_upvotes,
};
use crate::models::schema::politicians::dsl::{
    created_at as pol_created_at, politician_id as pol_id, politicians,
};
use crate::models::schema::user_reviews::dsl::{
    case_id, created_at as user_created_at, downvotes as review_downvotes, id as user_review_id,
    upvotes as review_upvotes, user_reviews,
};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseWithReviews {
    case: CorruptionCase,
    reviews: Vec<UserReview>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseWithReviewCount {
    case: CorruptionCase,
    reviews: i32,
}

pub struct Database {
    pub pool: DBPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoliticianCases {
    politician: Politician,
    corruption_cases: Option<Vec<CorruptionCase>>,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let result = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        Database { pool: result }
    }

    pub fn query_politicians(&self) -> Option<Vec<Politician>> {
        let pol_res = politicians
            .select(Politician::as_select())
            .order(pol_created_at.desc())
            .load(&mut self.pool.get().unwrap());
        match pol_res {
            Ok(politician_arr) => Some(politician_arr),
            Err(_err) => None,
        }
    }

    pub fn get_single_politician(&self, user_id: i32) -> Option<PoliticianCases> {
        let results: Option<Vec<(Politician, Option<CorruptionCase>)>> = politicians
            .left_join(corruption_cases.on(pol_id.eq(cor_id)))
            .filter(pol_id.eq(user_id))
            .select((
                Politician::as_select(),
                Option::<CorruptionCase>::as_select(),
            ))
            .load(&mut self.pool.get().unwrap())
            .optional()
            .unwrap();

        match results {
            Some(politician_with_cases) => {
                if let Some((politician, _)) = politician_with_cases.first() {
                    let mut cor_cases = politician_with_cases
                        .clone()
                        .into_iter()
                        .filter_map(|(_, corruption_case)| corruption_case)
                        .collect::<Vec<CorruptionCase>>();

                    cor_cases.sort_by(|a, b| b.created_at.cmp(&a.created_at));

                    Some(PoliticianCases {
                        politician: politician.clone(),
                        corruption_cases: Some(cor_cases),
                    })
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn create_corrupt_politician(
        &self,
        user: NewPolitician,
    ) -> Result<Politician, diesel::result::Error> {
        diesel::insert_into(politicians)
            .values(&user)
            .returning(Politician::as_returning())
            .get_results(&mut *self.pool.get().unwrap())?
            .first()
            .ok_or_else(|| diesel::result::Error::NotFound)
            .cloned()
    }
    // TODO: Enable deletion and updation feature
    // pub fn delete_politician(&self, find_id: i32) -> Result<usize, diesel::result::Error> {
    //     diesel::delete(politicians.filter(politician_id.eq(find_id)))
    //         .execute(&mut self.pool.get().unwrap())
    // }

    // pub fn update_politician(&self, user: Politician) -> Result<Politician, diesel::result::Error> {
    //     diesel::update(politicians.filter(politician_id.eq(user.politician_id)))
    //         .set(&user)
    //         .returning(Politician::as_returning())
    //         .get_result(&mut self.pool.get().unwrap())
    // }
    // Corruption cases db implementation is

    pub fn query_corruption_cases(&self) -> Vec<CorruptionCase> {
        let result: Vec<CorruptionCase> = corruption_cases
            .select(CorruptionCase::as_select())
            .order(cor_created_at.desc())
            .load(&mut self.pool.get().unwrap())
            .expect("Failed to query corruption cases");
        return result;
    }

    pub fn save_corruption_case(
        &self,
        case: NewCorruptionCase,
    ) -> Result<CorruptionCase, diesel::result::Error> {
        diesel::insert_into(corruption_cases)
            .values(&case)
            .returning(CorruptionCase::as_returning())
            .get_results(&mut *self.pool.get().unwrap())?
            .first()
            .ok_or_else(|| diesel::result::Error::NotFound)
            .cloned()
    }
    // user reviews
    pub fn user_reviews(&self, cases_id: i32) -> Option<CaseWithReviews> {
        let result: Option<Vec<(CorruptionCase, Option<UserReview>)>> = corruption_cases
            .left_join(user_reviews.on(case_id.eq(cor_case_id)))
            .select((
                CorruptionCase::as_select(),
                Option::<UserReview>::as_select(),
            ))
            .order(user_created_at.desc())
            .load(&mut self.pool.get().unwrap())
            .optional()
            .unwrap();
        result
            .map(|values| {
                let mut cases_with_reviews: HashMap<i32, CaseWithReviews> = HashMap::new();

                for (case, review) in values {
                    let entry = cases_with_reviews
                        .entry(case.id)
                        .or_insert(CaseWithReviews {
                            case,
                            reviews: Vec::new(),
                        });
                    if let Some(review) = review {
                        entry.reviews.push(review);
                    }
                }

                cases_with_reviews.get(&cases_id).cloned()
            })
            .flatten()
    }
    pub fn save_user_review(
        &self,
        user_review: NewUserReview,
    ) -> Result<UserReview, diesel::result::Error> {
        diesel::insert_into(user_reviews)
            .values(user_review)
            .returning(UserReview::as_returning())
            .get_results(&mut *self.pool.get().unwrap())?
            .first()
            .ok_or_else(|| diesel::result::Error::NotInTransaction)
            .cloned()
    }
    // UPVOTES AND DOWNVOTES FOR CASES
    pub fn upvote_case(&self, case_id_update: i32) -> Result<usize, diesel::result::Error> {
        diesel::update(corruption_cases.filter(cor_case_id.eq(case_id_update)))
            .set(case_upvotes.eq(case_upvotes + 1))
            .execute(&mut *self.pool.get().unwrap())
    }
    pub fn downvote_case(&self, case_id_update: i32) -> Result<usize, diesel::result::Error> {
        diesel::update(corruption_cases.filter(cor_case_id.eq(case_id_update)))
            .set(case_downvotes.eq(case_downvotes + 1))
            .execute(&mut *self.pool.get().unwrap())
    }
    // UPVOTES AND DOWNVOTES FOR user comments/reviews
    pub fn upvote_review(&self, case_id_update: i32) -> Result<usize, diesel::result::Error> {
        diesel::update(user_reviews.filter(user_review_id.eq(case_id_update)))
            .set(review_upvotes.eq(review_upvotes + 1))
            .execute(&mut *self.pool.get().unwrap())
    }
    pub fn downvote_review(&self, case_id_update: i32) -> Result<usize, diesel::result::Error> {
        diesel::update(user_reviews.filter(user_review_id.eq(case_id_update)))
            .set(review_downvotes.eq(review_downvotes + 1))
            .execute(&mut *self.pool.get().unwrap())
    }
    // upvotes
    pub fn cases_rating(&self) -> Result<Vec<CorruptionCase>, diesel::result::Error> {
        let result = corruption_cases
            .select(CorruptionCase::as_select())
            .order(case_upvotes.desc())
            .load(&mut self.pool.get().unwrap());
        result
    }
}
