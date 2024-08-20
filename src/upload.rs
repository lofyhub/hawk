use actix_multipart::{Field, Multipart};
use actix_web::dev::Payload;
use actix_web::web::Bytes;
use actix_web::ResponseError;
use actix_web::{FromRequest, HttpRequest};
use base64::{engine::general_purpose, Engine as _};
use cloudinary::upload::{result::UploadResult, Source, Upload, UploadOptions};
use crate::models::politicians::NewPolitician;
use futures::TryStreamExt;
use std::ffi::OsStr;
use std::fmt;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;

const MAX_IMAGE_SIZE: usize = 5 * 1024 * 1024; // 5 MB
const _MAX_NUM_FILES: u8 = 1;

pub struct MultipartRequestWithFile {
    pub file_name: String,
    pub file_content: Vec<Bytes>,
    pub politician: NewPolitician,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidationError {
    name: String,
    message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in field '{}': {}", self.name, self.message)
    }
}

impl ResponseError for ValidationError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }
}

impl MultipartRequestWithFile {
    pub async fn from_multipart(
        mut multipart: Multipart,
    ) -> Result<Self, <Self as FromRequest>::Error> {
        // The request might not contain any of the parameters, so need to check their presence
        let mut file_name: Option<String> = None;
        let mut file_content: Option<Vec<Bytes>> = None;
        let mut politician_data: Option<NewPolitician> = None;

        let mut bytes_len: usize = 0;
        // Iterate over all the multipart fields
        while let Ok(Some(mut field)) = multipart.try_next().await {
            println!("This is a multipart field: {:?}", &field);

            // Ensure content disposition is available
            if let Some(content_disposition) = field.content_disposition() {
                if let Some(field_name) = content_disposition.get_name() {
                    match field_name {
                        "json" => {
                            let json_data = Self::read_string(&mut field).await;
                            if let Some(json_string) = json_data {
                                // Deserialize the JSON string into your struct
                                let parsed_json: Result<NewPolitician, _> =
                                    serde_json::from_str(&json_string);
                                match parsed_json {
                                    Ok(json_struct) => {
                                        politician_data = Some(json_struct);
                                    }
                                    Err(e) => {
                                        println!("Failed to parse JSON: {:?}", e);
                                        return Err(actix_web::error::ErrorBadRequest(
                                            "Invalid JSON data",
                                        ));
                                    }
                                }
                            } else {
                                println!("No JSON data found");
                            }
                        }
                        "file" => {
                            file_name = content_disposition.get_filename().map(String::from);
                            let mut content = Vec::new();

                            while let Ok(Some(value)) = field.try_next().await {
                                bytes_len += value.len();

                                if bytes_len > MAX_IMAGE_SIZE {
                                    let error_message = format!(
                                        "The image file is too large, Expected {}",
                                        MAX_IMAGE_SIZE
                                    );
                                    return Err(actix_web::error::ErrorForbidden(error_message));
                                }
                                content.push(value)
                            }
                            file_content = Some(content);
                        }
                        // Ignore other parameters
                        _ => (),
                    }
                }
            } else {
                println!("Field without content disposition: {:?}", field);
            }
        }

        // Assert the required fields are present
        if politician_data.is_some() && file_name.is_some() && file_content.is_some() {
            Ok(MultipartRequestWithFile {
                file_name: file_name.unwrap(),
                file_content: file_content.unwrap(),
                politician: politician_data.unwrap(),
            })
        } else {
            let error = ValidationError {
                name: "other_param".to_string(),
                message: "Invalid request".to_string(),
            };
            Err(error.into()) // TODO - create a custom validation error which implements the actix_web::ResponseError trait
        }
    }

    pub async fn read_string(field: &mut Field) -> Option<String> {
        let bytes = field.try_next().await;

        if let Ok(Some(bytes)) = bytes {
            String::from_utf8(bytes.to_vec()).ok()
        } else {
            None
        }
    }
    pub async fn upload_to_cloudinary(&self) -> UploadResult {
        let image_extention = get_extension_from_filename(&self.file_name).unwrap();
        let mut bytes_slice = Vec::new();

        for chunk in &self.file_content {
            bytes_slice.extend_from_slice(chunk)
        }
        let image_base64 = general_purpose::STANDARD.encode(bytes_slice);

        let image_data_uri = format!("data:image/{};base64,{}", image_extention, image_base64);

        upload_img_to_cloudinary(image_data_uri, self.file_name.to_string()).await
    }
}

// By implementing FromRequest, we can directly declare our struct as a parameter for an actix handler function
impl FromRequest for MultipartRequestWithFile {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // get a future for a Multipart struct from the request
        let multipart_future = Multipart::from_request(req, payload);

        // As this is not an async function, we cannot use 'await'.
        // Instead, we create future from this async block and return a pinned Box containing our future.
        // This is because currently, traits cannot declare async functions, so instead the FromRequest trait declares a non-async function which returns a Future instead.
        let future = async {
            // Inside of this async block we are able to use 'await'
            let multipart = multipart_future.await?;

            // Await our async function containing the actual logic
            Self::from_multipart(multipart).await
        };
        Box::pin(future)
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut Payload::None)
    }
}

async fn upload_img_to_cloudinary(data_url: String, file_name: String) -> UploadResult {
    let options = UploadOptions::new().set_public_id(file_name);
    let upload = Upload::new(
        "824696885955381".to_string(),
        "deye3gicq".to_string(),
        "rRS6znH_5wvEhcZzUYZNqi7zXt0".to_string(),
    );
    let result = upload.image(Source::DataUrl(data_url), &options).await;
    result.unwrap()
}

fn get_extension_from_filename(filename: &str) -> Option<String> {
    println!("file name is{}", filename);
    let file_ext = Path::new(filename).extension().and_then(OsStr::to_str);
    match file_ext {
        Some(ext) => Some(ext.to_string()),
        None => {
            println!("error finding file extention: {}", filename);
            None
        }
    }
}
