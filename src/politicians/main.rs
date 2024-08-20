use crate::upload::MultipartRequestWithFile;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_validator::Json;
use cloudinary::upload::result::UploadResult::{Error as CloudinaryError, Success};
use models::politicians::NewPolitician;
use repository::database::Database;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::utils::{ErrorResponse, SuccessResponse};
use crate::{models, repository};

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    user_id: i32,
}

#[get("/politicians/{user_id}")]
async fn get_corrupt_politician(db: web::Data<Database>, info: web::Path<Info>) -> impl Responder {
    let corrupt_politician = db.get_single_politician(info.user_id);
    match corrupt_politician {
        Some(politician) => {
            let response = SuccessResponse::new_single(politician);
            HttpResponse::Ok().json(response)
        }
        None => {
            let error_message = format!("Politician with ID {} Not Found", info.user_id);
            let error_res = ErrorResponse::new(error_message);
            HttpResponse::NotFound().json(error_res)
        }
    }
}

#[get("/politicians")]
async fn get_corrupt_politicians(db: web::Data<Database>) -> impl Responder {
    let corrupt_politicians = db.query_politicians();
    match corrupt_politicians {
        Some(values) => {
            let response = SuccessResponse::new_multiple(values);
            HttpResponse::Ok().json(response)
        }
        None => HttpResponse::Ok().json(json!({
            "status": true,
            "data":[],
            "message": String::from("No data found"),
        })),
    }
}

#[post("/politicians")]
async fn save_corrupt_politicians(
    payload: MultipartRequestWithFile,
    db: web::Data<Database>,
) -> impl Responder {
    let corrupt_politician = payload.politician.clone();

    match corrupt_politician.validate() {
        Ok(_) => (),
        Err(err) => {
            let error_message =
                format!("some error occured while validating json payload: {}", err);
            println!("some error occured while validating json payload: {}", err);
            return HttpResponse::BadRequest().json(error_message);
        }
    };

    match payload.upload_to_cloudinary().await {
        Success(response) => {
            let image_url_str = response.secure_url;

            let mut filty_politician = corrupt_politician;
            filty_politician.photo_url = Some(image_url_str);

            let politician = db.create_corrupt_politician(filty_politician);

            println!("{:?}", politician);
            match politician {
                Ok(politician) => {
                    let success_response = SuccessResponse::new_single(politician);
                    return  HttpResponse::Created().json(success_response);
                }
                Err(err) => {
                    let error_message = format!("{}", err);
                    let error_res = ErrorResponse::new(error_message);
                    return HttpResponse::InternalServerError().json(error_res);
                }
            }
        }
        CloudinaryError(value) => {
            let error_message = format!("Failed to upload to cloudinary: {:?}", value);
            let error_res = ErrorResponse::new(error_message);
            return  HttpResponse::BadRequest().json(error_res);
        }
    }
}

#[get("/")]
async fn health() -> impl Responder {
    // Create a JSON response
    HttpResponse::Ok().json(json!({
        "app_name": "Politas",
        "status": "healthy",
        "message": "Why don't programmers like nature? It has too many bugs.",
    }))
}
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
