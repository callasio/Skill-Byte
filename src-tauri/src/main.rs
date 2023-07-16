// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, State};
use tokio::sync::Mutex;
use crate::mutex::option::unwrap;
use crate::selenium::chrome_driver::ChromeDriver;
use crate::selenium::session::DriverSession;

mod selenium;
mod codeforces;
pub mod mutex;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start_driver(
    chrome_driver_state: State<'_, Mutex<Option<ChromeDriver>>>,
    driver_session_state: State<'_, DriverSession>
) -> Result<(), error::ProjectError> {
    *chrome_driver_state.lock().await = Some(ChromeDriver::new().await?);

    mutex::option::unwrap!(chrome_driver_state, {
        let port_num = chrome_driver_state.start().await?;
        driver_session_state.start(selenium::session::Port(port_num)).await?;
    });

    Ok(())
}

#[tauri::command]
async fn exit(
    driver_session_state: State<'_, DriverSession>
) -> Result<(), error::ProjectError> {
    driver_session_state.exit().await?;

    Ok(())
}

fn main() {
    tauri::Builder::default()

        .manage(DriverSession::default())
        .manage(Mutex::<Option<ChromeDriver>>::new(None))
        .invoke_handler(tauri::generate_handler![greet, start_driver, exit])
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
