use tokio::process;
use super::error::ExecutionError;

impl super::ChromeDriver {
    pub async fn execute(&self) -> Result<(), ExecutionError> {
        let mut command = process::Command::new(&self.execution_file_path);
        command.kill_on_drop(true);

        let mut process_guard = self.execution_process.lock().await;
        *process_guard = Some(command.spawn()?);

        Ok(())
    }
}