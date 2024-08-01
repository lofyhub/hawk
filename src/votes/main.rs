use actix_web::{post, web, HttpResponse, Responder};

use repository::database::Database;
use serde::{Deserialize, Serialize};

use crate::repository;
use crate::utils::{ErrorResponse, SuccessResponse};

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    case_id: i32,
}

#[post("/case/upvote/{case_id}")]
async fn save_case_upvote(db: web::Data<Database>, info: web::Path<Info>) -> impl Responder {
    let result = db.upvote_case(info.case_id);
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

#[post("/case/downvote/{case_id}")]
async fn save_case_downvote(db: web::Data<Database>, info: web::Path<Info>) -> impl Responder {
    let result = db.downvote_case(info.case_id);
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

// USER REVIEWS UPVOTES AND DOWNVOTES

#[post("/review/upvote/{case_id}")]
async fn save_review_upvote(db: web::Data<Database>, info: web::Path<Info>) -> impl Responder {
    let result = db.upvote_review(info.case_id);
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

#[post("/review/downvote/{case_id}")]
async fn save_review_downvote(db: web::Data<Database>, info: web::Path<Info>) -> impl Responder {
    let result = db.downvote_review(info.case_id);
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
