use actix_web::{get, post, web, HttpResponse, Responder};

use actix_web_validator::Json;
use models::user_reviews::NewUserReview;
use repository::database::Database;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::{ErrorResponse, SuccessResponse};
use crate::{models, repository};

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    case_id: i32,
}

#[get("/report/{case_id}")]
async fn get_case_review(db: web::Data<Database>, info: web::Path<Info>) -> impl Responder {
    let results = db.user_reviews(info.case_id);
    match results {
        Some(values) => HttpResponse::Ok().json(values),
        None => {
            let error_message = format!("Reviews for case ID {} Not Found", info.case_id);
            let error_res = ErrorResponse::new(error_message);
            HttpResponse::NotFound().json(error_res)
        }
    }
}

#[post("/report")]
async fn save_case_review(
    db: web::Data<Database>,
    user_review: Json<NewUserReview>,
) -> impl Responder {
    match user_review.validate() {
        Ok(_) => (),
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    }

    let result = db.save_user_review(user_review.into_inner());
    match result {
        Ok(values) => {
            let success_response = SuccessResponse::new_single(values);
            HttpResponse::Created().json(success_response)
        }
        Err(err) => {
            let error_message = format!("{}", err);
            let error_res = ErrorResponse::new(error_message);
            HttpResponse::InternalServerError().json(error_res)
        }
    }
}
