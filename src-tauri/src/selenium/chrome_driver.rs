#![allow(unused)]

pub struct ChromeDriver {
    pub latest_release_version: String,
    pub download_url: String
}

#[derive(Debug, thiserror::Error)]
pub enum ChromeDriverError {
    #[error("IoError while getting latest release version using curl")]
    ConstructCurlIoError(#[from] std::io::Error),
    #[error("FromUtf8Error while parsing utf8 string")]
    ConstructParseError(#[from] std::string::FromUtf8Error),
    #[error("Unsupported OS")]
    ConstructUnsupportedOSError
}

#[doc = "Constructor"]
impl ChromeDriver {
    pub async fn new() -> Result<Self, ChromeDriverError> {
        let latest_release_version = Self::get_latest_release_version().await?;
        let download_url = Self::download_url(&latest_release_version)?;

        Ok(Self {
            latest_release_version,
            download_url
        })
    }

    async fn get_latest_release_version() -> Result<String, ChromeDriverError> {
        use tokio::process::Command;

        let curl_output = Command::new("curl")
            .arg("https://chromedriver.storage.googleapis.com/LATEST_RELEASE")
            .output()
            .await?;

        Ok(String::from_utf8(curl_output.stdout)?)
    }

    fn download_url(latest_release_version: &str) -> Result<String, ChromeDriverError> {
        Ok(format!(
            "https://chromedriver.storage.googleapis.com/{}/chromedriver_{}.zip",
            latest_release_version,
            Self::get_os_path()?
        ))
    }

    fn get_os_path() -> Result<String, ChromeDriverError> {
        if cfg!(windows) {
            Ok(String::from("win32"))
        } else if cfg!(linux) {
            Ok(String::from("linux64"))
        } else if cfg!(mac) {
            Ok(String::from("mac64"))
        } else {
            Err(ChromeDriverError::ConstructUnsupportedOSError)
        }
    }
}

static DOWNLOAD_PATH: &str = "./web_driver/";

#[cfg(target_os = "windows")]
static DOWNLOAD_FILE_NAME: &str = "chrome_driver.exe";
#[cfg(any(target_os = "macos", target_os = "linux"))]
static DOWNLOAD_FILE_NAME: &str = "chrome_driver";

#[doc = "Download"]
impl ChromeDriver {
    pub async fn download(&self) {
        use std::fs::File;

        let mut resp = reqwest::get(&self.download_url).await;
        let mut out = File::create(DOWNLOAD_PATH);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub static LATEST_RELEASE_VERSION: &str = "114.0.5735.90";

    #[tokio::test]
    async fn constructor() {
        let chrome_driver = ChromeDriver::new().await.unwrap();

        assert_eq!(chrome_driver.latest_release_version, LATEST_RELEASE_VERSION);
    }
}