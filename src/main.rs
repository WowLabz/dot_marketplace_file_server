#[macro_use]
extern crate rocket;

use rocket::http::{ContentType};
use rocket::data::Data;
use rocket::response::status;
use rocket::serde::json::Json;

mod handler;
mod service;
mod controller;
mod config;

use crate::controller::file_upload;
use crate::handler::{response::Response};


#[get("/")]
fn index() ->  &'static str {
    "Dot marketplace file server"
}

#[post("/upload-file", data = "<form_data>")]
async fn upload_file(
    content_type: &ContentType,
    form_data: Data<'_>,
    ) -> Result<status::Custom<Json<Response>>, status::Custom<Json<Response>>> {
        let response = file_upload::upload_file(content_type, form_data).await;
        response
}

#[launch]
fn rocket() -> _ {
    rocket::build()
            .mount("/", routes![index, upload_file])
}