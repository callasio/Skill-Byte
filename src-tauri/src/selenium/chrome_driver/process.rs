use tokio::process;
use super::error::ExecutionError;

impl super::ChromeDriver {
    pub async fn execute(&self) -> Result<(), ExecutionError> {
        let mut command = process::Command::new(&self.execution_file_path);

        let mut process_guard = self.execution_process.lock().await;
        *process_guard = Some(command.spawn()?);

        Ok(())
    }

    async fn terminate(&mut self) {
        if let Some(child_process) = &mut *self.execution_process.lock().await {
            child_process.kill()
                .await
                .unwrap_or_else(
                    |err| eprintln!("Failed to kill chromedriver process. Error: {}", err)
                );
        }
    }
}

impl Drop for super::ChromeDriver {
    fn drop(&mut self) {
        futures::executor::block_on(async {
            self.terminate().await;
        });
    }
}