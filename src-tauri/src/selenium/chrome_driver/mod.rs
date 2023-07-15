pub mod error;
mod install;
mod constructor;
mod process;

use error::*;
use tokio::{sync::Mutex, process::Child};

pub static DRIVER_PATH: &str = "./web_driver/";
pub static DOWNLOAD_FILE_NAME: &str = "driver_tmp.zip";

#[cfg(target_os = "windows")]
pub static DRIVER_FILE_NAME: &str = "chromedriver.exe";
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub static DRIVER_FILE_NAME: &str = "chromedriver";

pub struct ChromeDriver {
    download_url: String,
    execution_file_path: String,
    execution_process: Mutex<Option<Child>>
}

impl ChromeDriver {
    pub async fn start() -> Result<(), ChromeDriverError> {
        let chrome_driver = Self::new().await?;
        chrome_driver.install().await?;
        chrome_driver.execute().await?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn install() {
        let chrome_driver = ChromeDriver::new().await.unwrap();
        chrome_driver.install().await.unwrap();
    }
}