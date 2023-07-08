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
        let mut session_locked = session.lock().await;

        if session_locked.is_some() {
            session_locked.clone().unwrap().quit().await?;
        }

        *session_locked = Some(Self::new_driver("https://codeforces.com/").await?);

        Ok(())
    }

    async fn new_driver(url: &str) -> WebDriverResult<WebDriver> {
        use thirtyfour::DesiredCapabilities;

        let capability = DesiredCapabilities::chrome();

        WebDriver::new(url, capability).await
    }
}