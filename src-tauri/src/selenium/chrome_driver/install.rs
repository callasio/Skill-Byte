use super::error::DownloadError;

static DRIVER_PATH: &str = "./web_driver/";
static DOWNLOAD_FILE_NAME: &str = "driver.zip";

#[cfg(target_os = "windows")]
static DRIVER_FILE_NAME: &str = "chrome_driver.exe";
#[cfg(any(target_os = "macos", target_os = "linux"))]
static DRIVER_FILE_NAME: &str = "chrome_driver";

impl super::ChromeDriver {
    pub(super) async fn install(&self) -> Result<(), DownloadError> {
        if Self::is_installed() {
            return Ok(());
        }

        self.download().await?;
        self.extract()?;

        Ok(())
    }

    pub fn is_installed() -> bool {
        let file_path = format!("{DRIVER_PATH}{DRIVER_FILE_NAME}");

        std::fs::metadata(file_path).is_ok()
    }

    async fn download(&self) -> Result<(), DownloadError> {
        let _ = std::fs::create_dir(DRIVER_PATH);
        let mut out = std::fs::File::create(format!(
            "{DRIVER_PATH}{DOWNLOAD_FILE_NAME}"
        ))?;

        let resp = reqwest::get(&self.download_url).await?;
        let body_bytes = resp.bytes().await?;
        let mut body_ref = body_bytes.as_ref();

        std::io::copy(&mut body_ref, &mut out)?;

        Ok(())
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