use rocket::response::Responder;
use thiserror::Error;

#[derive(Responder)]
#[response(status = 400)]
pub struct APIErrorResponder {
    error_message: String,
}

impl From<APIError> for APIErrorResponder {
    fn from(error: APIError) -> Self {
        Self {
            error_message: error.to_string(),
        }
    }
}

#[derive(Error, Debug)]
pub enum APIError {
    #[error("not a valid date")]
    DateError(String),
    #[error("had trouble serializing this document to JSON")]
    JsonError,
    #[error("couldn't find this liturgy in our database")]
    LiturgyError(String),
}
