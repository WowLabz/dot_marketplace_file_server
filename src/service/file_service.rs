use rocket::data::Data;
use rocket::http::{ContentType};

use rocket_multipart_form_data::{
    mime::Mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions
};

use crate::handler::error_handler::TransmissionError;
pub struct MultipartHandler {
    pub content_type: Option<Mime>,
    pub file_name: String,
    pub raw: Vec<u8>
}

impl MultipartHandler {
    pub async  fn from(
        content_type: &ContentType,
        form_data: Data<'_>
    ) -> Result<Self, TransmissionError> {
        
        let multipart_option = MultipartFormDataOptions::with_multipart_form_data_fields(
            vec![ MultipartFormDataField::raw("filesent").size_limit(200 * 1024 * 1024) ],
        );

        let mut multipart_form_data = MultipartFormData::parse(&content_type, form_data, multipart_option).await?;

        let content = multipart_form_data
                                        .raw
                                        .remove("filesent")
                                        .ok_or_else(|| TransmissionError::Message(String::from("No data found in the file")))?;
        let filename = content[0]
                                    .file_name
                                    .clone()
                                    .ok_or_else(|| TransmissionError::Message(String::from("Could not get the filename")))?;
        Ok(Self{
            content_type: content[0].content_type.clone(),
            file_name: filename,
            raw: content[0].raw.clone()
        })
    }
}