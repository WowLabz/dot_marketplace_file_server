use rocket::data::Data;
use rocket::http::{ContentType, Status};
use rocket::response::status;
use rocket::serde::json::Json;


use crate::service::{file_service::MultipartHandler, nft_storage_service};
use crate::handler::{response::Response};
use crate::config::nft_storage_config::NftStorageConfig;

pub async fn upload_file(
    content_type: &ContentType,
    form_data: Data<'_>
)-> Result<status::Custom<Json<Response>>, status::Custom<Json<Response>>> {
    let multipart_result = MultipartHandler
                                                        ::from(content_type, form_data).await;
    let multipart = match multipart_result {
        Ok(data) => data,
        Err(_) => {
            let response = Response::unsuccessful("file upload failed");
            return Err(status::Custom(Status::BadRequest, Json(response)));
        }
    };
    let nft_storage_config = NftStorageConfig::default();
    let request_url = format!("{}{}",nft_storage_config.base, nft_storage_config.route);
    let nft_service_res = nft_storage_service::Client::new()
                                    .with_request_url(request_url)
                                    .upload_nft(multipart)
                                    .await;
    let file_ipfs_url = match nft_service_res {
        Ok(url) => url,
        Err(_) => {
            let response = Response::unsuccessful("file upload error");
            return Err(status::Custom(Status::InternalServerError, Json(response)));
        }
    };
    let message = format!("File upload success at ipfs url : {}",file_ipfs_url);
    let message = message.as_str();
    let response = Response::successful(message);
    Ok(status::Custom(Status::Accepted, Json(response)))
}