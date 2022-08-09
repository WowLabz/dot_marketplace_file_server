use rocket_multipart_form_data::MultipartFormDataError;

pub enum TransmissionError {
    RocketError(rocket::Error),
    MultipartError(MultipartFormDataError),
    Message(String)
}

impl From<rocket::Error> for TransmissionError {
    fn from(error: rocket::Error) -> Self{
        Self::RocketError(error)
    }
}

impl From<MultipartFormDataError> for TransmissionError {
    fn from(error: MultipartFormDataError) -> Self {
        Self::MultipartError(error)
    }
}

impl From<String> for TransmissionError {
    fn from(error: String) -> Self {
        Self::Message(error)
    }
}