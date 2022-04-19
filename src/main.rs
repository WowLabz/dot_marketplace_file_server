#[macro_use] extern crate rocket;
use rocket::data::Data;
use rocket::http::{ContentType, Status};
use rocket::response::status;
use rocket::serde::json::Json;


mod utils;
mod file_uploader;
use utils::data_parser::{MultipartHandler, TheResponse};
use file_uploader::nft_storage;




#[get("/")]
fn index() -> &'static str {
    "hello"
}

#[post("/upload-file", data = "<form_data>")]
async fn upload_file(
    content_type: &ContentType, 
    form_data: Data<'_>
) -> Result<status::Custom<Json<TheResponse>>, status::Custom<Json<TheResponse>>> {
    let multipart = match MultipartHandler::from(content_type, form_data).await {
        Some(data) => data,
        None => {
            let message = Json(TheResponse::new(false, "Invalid file provided or file not provided"));
            return Err(status::Custom(Status::BadRequest, message))
        },
    };
    // println!("{:?}",multipart);
    let url = format!("https://api.nft.storage/upload");
    let nft_storage_res = nft_storage::Client::new()
                                    .with_request_url(url)
                                    .upload_nft(multipart)
                                    .await;
    let nft_storage = match nft_storage_res {
        Ok(value) => value,
        Err(_) => {
            let message = Json(TheResponse::new(false, "error uploading file to ipfs"));
            return Err(status::Custom(Status::BadGateway, message))
        }
    };
    let message = Json(TheResponse::new(true, nft_storage.as_str()));
    Ok(status::Custom(Status::Accepted, message))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
            .mount("/", routes![index, upload_file])
}