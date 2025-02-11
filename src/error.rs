use std::fmt::Display;

use derive_more::From;
use leptos::prelude::ServerFnError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    SensorError(String),
    ServerFnError(ServerFnError),
    IoError(std::io::Error),
    SerdeJsonError(serde_json::Error),
    #[from]
    ReqwestError(reqwest::Error),
    Other(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SensorError(e) => write!(f, "Sensor error: {e}"),
            Self::ServerFnError(e) => write!(f, "Server function error: {e}"),
            Self::IoError(e) => write!(f, "IO error: {e}"),
            Self::SerdeJsonError(e) => write!(f, "Serde JSON error: {e}"),
            Self::ReqwestError(e) => write!(f, "Reqwest error: {e}"),
            Self::Other(e) => write!(f, "Other error: {e}"),
        }
    }
}
