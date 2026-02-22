#[derive(thiserror::Error, Debug)]
pub enum GeminiResponseError {
    #[error(transparent)]
    WreqError(wreq::Error),
    #[error("Response status not Ok. Response string: {0}")]
    ///Response status not Ok. Contains Response string
    StatusNotOk(String),
    #[error("Cannot Respond if last Chat has Role::Model")]
    ///Cannot Responnd if last Chat has Role::Model
    NothingToRespond,
}

#[derive(thiserror::Error, Debug)]
pub enum GeminiResponseStreamError {
    #[error(transparent)]
    WreqError(wreq::Error),
    #[error("Invalid Response Format received. Response: {0}")]
    ///Invalid Response Format received. Contains response string
    InvalidResposeFormat(String),
}
