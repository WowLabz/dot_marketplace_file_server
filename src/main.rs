#[macro_use]
extern crate rocket;

use rocket::http::{ContentType, Header};
use rocket::data::Data;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::fairing::{Fairing, Info, Kind };
use rocket::{ Request, Response };


mod handler;
mod service;
mod controller;
mod config;

use crate::controller::file_upload;
use crate::handler::{response::ResponseSent};


#[get("/")]
fn index() ->  &'static str {
    "Dot marketplace file server"
}

#[post("/upload-file", data = "<form_data>")]
async fn upload_file(
    content_type: &ContentType,
    form_data: Data<'_>,
    ) -> Result<status::Custom<Json<ResponseSent>>, status::Custom<Json<ResponseSent>>> {
        let response = file_upload::upload_file(content_type, form_data).await;
        response
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
            .mount("/", routes![index, upload_file])
            .attach(CORS)
}