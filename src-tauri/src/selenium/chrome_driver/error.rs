
#![allow(clippy::enum_variant_names)]

#[derive(Debug, thiserror::Error)]
pub enum ChromeDriverError {
    #[error("While constructing chrome driver instance")]
    ConstructError(#[from] ConstructError),
    #[error("While downloading chrome driver from google api")]
    DownloadError(#[from] DownloadError)
}

#[derive(Debug, thiserror::Error)]
pub enum ConstructError {
    #[error("IoError while getting latest release version using curl")]
    CurlIoError(#[from] std::io::Error),
    #[error("FromUtf8Error while parsing utf8 string")]
    ParseError(#[from] std::string::FromUtf8Error),
    #[error("Unsupported OS")]
    UnsupportedOSError,
}

#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("Error while creating chrome driver file")]
    FileCreateError(#[from] std::io::Error),
    #[error("Error while downloading chrome driver with http request")]
    HttpRequestError(#[from] reqwest::Error),
    #[error("Error while extracting zip file.")]
    ExtractZipError(#[from] zip::result::ZipError)
}
