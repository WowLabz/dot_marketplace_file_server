use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ResponseSent {
    pub success: bool,
    pub message: Option<String>,
    pub error: Option<String>
}

impl ResponseSent {
    pub fn successful(success_message: &str) -> Self {
        Self{
            success: true,
            message: Some(String::from(success_message)),
            error: None
        }
    }

    pub fn unsuccessful(error_message: &str) -> Self {
        Self{
            success: false,
            message: None,
            error: Some(String::from(error_message))
        }
    }
}