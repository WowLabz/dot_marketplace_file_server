use rocket::data::Data;
use rocket::http::ContentType;
use rocket_multipart_form_data::{
    mime::Mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use rocket::serde::{Serialize, };



#[derive(Debug)]
pub struct MultipartHandler {
    pub content_type: Option<Mime>,
    pub file_name: String,
    pub raw: Vec<u8>
}

#[derive(Serialize)]
pub struct TheResponse {
    pub success: bool,
    pub message: String
}


impl MultipartHandler {

    pub async fn from(
        content_type: &ContentType,
        form_data: Data<'_>
    ) -> Option<Self> {
        let option_multipart = MultipartFormDataOptions::with_multipart_form_data_fields(
            vec![MultipartFormDataField::raw("filesent").size_limit(200 * 1024 * 1024)]
        );
        let mutlipart_form_data_result = MultipartFormData::parse(&content_type, form_data, option_multipart).await;
        let mut mutlipart_form_data = match mutlipart_form_data_result {
            Ok(data) => data,
            Err(_) => return None
        };
        let content_option = mutlipart_form_data
                                        .raw
                                        .remove("filesent");
        
        let content = match content_option {
            Some(data) => data,
            None => return None
        };
        let filename = match content[0].file_name.clone() {
            Some(name) => name,
            None => return None
        };
        Some(Self{
            content_type: content[0].content_type.clone(),
            file_name: filename,
            raw: content[0].raw.clone(),
        }
        )
    }
}

impl TheResponse {
    pub fn new(success: bool, message: &str) -> Self {
        Self {
            success: success,
            message: format!("{}",message)
        }
    }
}

