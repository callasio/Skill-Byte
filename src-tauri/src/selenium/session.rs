use thirtyfour::error::WebDriverResult;
use tokio::sync::Mutex;
use thirtyfour::WebDriver;

#[derive(Default)]
pub(crate) struct DriverSession {
    problem: Mutex<Option<WebDriver>>,
    contest: Mutex<Option<WebDriver>>,
    standing: Mutex<Option<WebDriver>>
}

impl DriverSession {
    pub(crate) async fn start(&self) -> WebDriverResult<()> {
        let futures = vec![
            Self::start_driver_session(&self.problem),
            Self::start_driver_session(&self.contest),
            Self::start_driver_session(&self.standing)
        ];

        for future in futures {
            future.await?;
        }

        Ok(())
    }

    async fn start_driver_session(
        session: &Mutex<Option<WebDriver>>
    ) -> WebDriverResult<()> {
        let mut session_guard = session.lock().await;

        if session_guard.is_some() {
            unreachable!();
        }

        *session_guard = Some(Self::new_driver("localhost:1420").await?);

        Ok(())
    }

    async fn new_driver(url: &str) -> WebDriverResult<WebDriver> {
        use thirtyfour::DesiredCapabilities;

        let capability = DesiredCapabilities::chrome();

        WebDriver::new(url, capability).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_driver() {
        use thirtyfour::DesiredCapabilities;
        
        crate::selenium::chrome_driver::ChromeDriver::start().await.unwrap();

        let mut caps = DesiredCapabilities::chrome();
        caps.add_chrome_arg("--headless").unwrap();

        let driver = WebDriver::new("http://localhost:9515", caps).await.unwrap();

        driver.goto("https://codeforces.com/").await.unwrap();
        let title = driver.title().await.unwrap();
        assert_eq!(title, "Codeforces");

        driver.goto("https://www.acmicpc.net/").await.unwrap();
        let title = driver.title().await.unwrap();
        assert_eq!(title, "Baekjoon Online Judge");
    }
}