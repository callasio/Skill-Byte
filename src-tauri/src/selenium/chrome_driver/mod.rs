#![allow(unused)]

pub mod error;

use std::fmt::{Display, Formatter};
use error::*;

pub struct ChromeDriver {
    latest_release_version: String,
    download_url: String,
    os: String
}

impl ChromeDriver {
    pub async fn new() -> Result<Self, ConstructError> {
        let latest_release_version = Self::get_latest_release_version().await?;
        let download_url = Self::download_url(&latest_release_version)?;
        let os = Self::get_os()?;

        Ok(Self {
            latest_release_version,
            download_url,
            os
        })
    }

    async fn get_latest_release_version() -> Result<String, ConstructError> {
        use tokio::process::Command;

        let curl_output = Command::new("curl")
            .arg("https://chromedriver.storage.googleapis.com/LATEST_RELEASE")
            .output()
            .await?;

        Ok(String::from_utf8(curl_output.stdout)?)
    }

    fn download_url(latest_release_version: &str) -> Result<String, ConstructError> {
        Ok(format!(
            "https://chromedriver.storage.googleapis.com/{}/chromedriver_{}.zip",
            latest_release_version,
            Self::get_os()?
        ))
    }

    fn get_os() -> Result<String, ConstructError> {
        if cfg!(windows) {
            Ok(String::from("win32"))
        } else if cfg!(linux) {
            Ok(String::from("linux64"))
        } else if cfg!(mac) {
            Ok(String::from("mac64"))
        } else {
            Err(ConstructError::UnsupportedOSError)
        }
    }
}

static DRIVER_PATH: &str = "./web_driver/";
static DOWNLOAD_FILE_NAME: &str = "driver.zip";

#[cfg(target_os = "windows")]
static DRIVER_FILE_NAME: &str = "chrome_driver.exe";
#[cfg(any(target_os = "macos", target_os = "linux"))]
static DRIVER_FILE_NAME: &str = "chrome_driver";

impl ChromeDriver {
    pub async fn install(&self) -> Result<(), DownloadError> {
        let file = self.download().await?;

        Ok(())
    }

    async fn download(&self) -> Result<std::fs::File, DownloadError> {
        std::fs::create_dir(DRIVER_PATH);
        let mut out = std::fs::File::create(format!(
            "{DRIVER_PATH}{DOWNLOAD_FILE_NAME}"
        ))?;

        let resp = reqwest::get(&self.download_url).await?;
        let body_bytes = resp.bytes().await?;
        let mut body_ref = body_bytes.as_ref();

        std::io::copy(&mut body_ref, &mut out)?;

        Ok(out)
    }

    fn extract(&self) -> Result<(), DownloadError> {
        let file_read = std::fs::File::open(
            format!("{DRIVER_PATH}{DOWNLOAD_FILE_NAME}")
        ).unwrap();

        let mut zip_arch = zip::ZipArchive::new(file_read)?;
        zip_arch.extract(DRIVER_PATH)?;

        Ok(())
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

    #[tokio::test]
    async fn install() {
        let chrome_driver = ChromeDriver::new().await.unwrap();
        let file = chrome_driver.download().await.unwrap();
        chrome_driver.extract().unwrap();
    }
}