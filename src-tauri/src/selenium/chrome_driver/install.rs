use super::error::DownloadError;

impl super::ChromeDriver {
    pub(super) async fn install(&self) -> Result<(), DownloadError> {
        if self.is_installed() {
            return Ok(());
        }

        self.download().await?;
        self.extract()?;

        Ok(())
    }

    pub fn is_installed(&self) -> bool {
        std::fs::metadata(&self.execution_file_path).is_ok()
    }

    async fn download(&self) -> Result<(), DownloadError> {
        let _ = std::fs::create_dir(super::DRIVER_PATH);
        let mut out = std::fs::File::create(format!(
            "{}{}", super::DRIVER_PATH, super::DOWNLOAD_FILE_NAME
        ))?;

        let resp = reqwest::get(&self.download_url).await?;
        let body_bytes = resp.bytes().await?;
        let mut body_ref = body_bytes.as_ref();

        std::io::copy(&mut body_ref, &mut out)?;

        Ok(())
    }

    fn extract(&self) -> Result<(), DownloadError> {
        let file_read = std::fs::File::open(
            format!("{}{}", super::DRIVER_PATH, super::DOWNLOAD_FILE_NAME)
        ).unwrap();

        let mut zip_arch = zip::ZipArchive::new(file_read)?;
        zip_arch.extract(super::DRIVER_PATH)?;

        Ok(())
    }
}