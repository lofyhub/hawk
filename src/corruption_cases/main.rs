use actix_web::{get, post, web, HttpResponse, Responder};
use repository::database::Database;

use crate::models::cases::NewCorruptionCase;
use crate::repository;
use crate::utils::{ErrorResponse, SuccessResponse};

#[get("/corruption_cases")]
async fn get_corruption_cases(db: web::Data<Database>) -> impl Responder {
    let corruption_cases = db.query_corruption_cases();
    let response = SuccessResponse::new_multiple(corruption_cases);
    HttpResponse::Ok().json(response)
}

#[get("/cases/ratings")]
async fn get_most_upvoted_cases(db: web::Data<Database>) -> impl Responder {
    let cases_rating = db.cases_rating();
    match cases_rating {
        Ok(values) => {
            let response = SuccessResponse::new_multiple(values);
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            let error_message = format!("{}", err);
            let error_response = ErrorResponse::new(error_message);
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

#[post("/corruption_cases")]
async fn save_corruption_case(
    db: web::Data<Database>,
    corrupt_case: web::Json<NewCorruptionCase>,
) -> impl Responder {
    let saved_case = db.save_corruption_case(corrupt_case.into_inner());
    println!("{:?}", saved_case);
    match saved_case {
        Ok(case) => {
            let success_response = SuccessResponse::new_single(case);
            HttpResponse::Ok().json(success_response)
        }
        Err(err) => {
            let error_message = format!("{}", err);
            let error_response = ErrorResponse::new(error_message);
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}
