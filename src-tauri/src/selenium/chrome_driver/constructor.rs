use super::error::ConstructError;

impl super::ChromeDriver {
    pub(super) async fn new() -> Result<Self, ConstructError> {
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