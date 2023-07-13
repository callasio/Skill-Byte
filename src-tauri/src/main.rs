// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::State;
use crate::selenium::chrome_driver::ChromeDriver;
use crate::selenium::session::DriverSession;

mod selenium;
mod codeforces;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start_driver(driver_session_state: State<'_, DriverSession>) -> Result<(), error::ProjectError> {
    ChromeDriver::start().await?;
    driver_session_state.start().await?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(DriverSession::default())
        .invoke_handler(tauri::generate_handler![greet, start_driver])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod error {
    use serde::Serializer;
    use thirtyfour::error::WebDriverError;
    use crate::selenium::chrome_driver::error::ChromeDriverError;

    #[derive(Debug, thiserror::Error)]
    pub enum ProjectError {
        #[error("Error occurred in chrome driver.")]
        ChromeDriverError(#[from] ChromeDriverError),
        #[error("Error occurred in web driver code.")]
        WebDriverError(#[from] WebDriverError)
    }

    impl serde::Serialize for ProjectError {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer
        {
            serializer.serialize_str(self.to_string().as_ref())
        }
    }
}
