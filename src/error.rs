use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type AppRes<T> = std::result::Result<T, AppError>;
pub type StrRes<T> = std::result::Result<T, String>;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AppError {
    #[error("Index out of bounds \nLine {0}")]
    IndexErr(u16),
    #[error("DB Error: {1} \nLine {0}")]
    DBErr(u16, String),
    #[error("Unknown state: {1} \nLine {0}")]
    UnknownState(u16, String),
    #[error("No current routine \nLine {0}")]
    NotFound(u16),
    #[error("Validation error: {1} \nLine{0}")]
    ValidationErr(u16, String),
    #[error("HTTP error: {1} \nLine{0}")]
    HttpErr(u16, String),
}
