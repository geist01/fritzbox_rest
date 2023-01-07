use thiserror::Error;

#[derive(Error, Debug)]
pub enum FritzboxError {
    #[error("IO")]
    Network(#[from] std::io::Error),

    #[error("IO")]
    Reqwest(#[from] reqwest::Error),

    #[error("IO")]
    SerdeXml(#[from] serde_xml_rs::Error),

    #[error("IO")]
    SerdeJson(#[from] serde_json::Error),

    #[error("IO")]
    ParseUrl(#[from] reqwest::UrlError),

    #[error("IO")]
    ParseResponse(#[from] std::num::ParseIntError),

    #[error("IO")]
    MissingParameter(String),
}
