pub mod error;
mod install;
mod constructor;

use error::*;

#[allow(dead_code)]
pub struct ChromeDriver {
    latest_release_version: String,
    download_url: String,
    os: String
}

impl ChromeDriver {
    pub async fn start() -> Result<(), ChromeDriverError> {
        let chrome_driver = Self::new().await?;
        chrome_driver.install().await?;

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
        chrome_driver.install().await.unwrap();
    }
}