use thirtyfour::error::WebDriverResult;
use tokio::sync::Mutex;
use thirtyfour::WebDriver;
use crate::mutex::option::unwrap;

#[derive(Debug, Copy, Clone, Default)]
pub struct Port(pub u32);

impl Port {
    fn to_url(self) -> String {
        println!("http://localhost:{}", self.0);
        format!("http://localhost:{}", self.0)
    }
}

#[derive(Default)]
pub struct DriverSession {
    pub port: Mutex<Port>,
    problem: Mutex<Option<WebDriver>>,
    contest: Mutex<Option<WebDriver>>,
    standing: Mutex<Option<WebDriver>>
}

impl DriverSession {
    pub async fn start(&self, port: Port) -> WebDriverResult<()> {
        *self.port.lock().await = port;
        let host_url = port.to_url();

        Self::start_driver_session(&self.problem, &host_url).await?;
        Self::start_driver_session(&self.contest, &host_url).await?;
        Self::start_driver_session(&self.standing, &host_url).await?;

        Ok(())
    }

    async fn start_driver_session(
        session: &Mutex<Option<WebDriver>>,
        host_url: &str
    ) -> WebDriverResult<()> {
        *session.lock().await = Some(Self::new_driver(host_url).await?);

        unwrap!(session, {
            session.goto("https://codeforces.com/").await?;
        });

        Ok(())
    }

    async fn new_driver(host_url: &str) -> WebDriverResult<WebDriver> {
        use thirtyfour::DesiredCapabilities;

        let mut capabilities = DesiredCapabilities::chrome();
        // capabilities.add_chrome_arg("--headless")?;

        WebDriver::new(host_url, capabilities).await
    }

    pub async fn exit(&self) -> WebDriverResult<()> {
        let problem = &self.problem;
        let contest = &self.contest;
        let standing = &self.standing;

        unwrap!(problem, {
            problem.clone().quit().await?
        });

        unwrap!(contest, {
            contest.clone().quit().await?
        });

        unwrap!(standing, {
            standing.clone().quit().await?
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selenium::chrome_driver::ChromeDriver;

    #[tokio::test]
    async fn start_driver() {
        let chrome_driver = ChromeDriver::new().await.unwrap();
        let port_num = chrome_driver.start().await.unwrap();
        let driver = DriverSession::default();
        driver.start(Port(port_num)).await.unwrap();

        let contest = &driver.contest;

        unwrap!(contest, {
            assert_eq!(contest.title().await.unwrap(), "Codeforces");
            contest.goto("https://www.acmicpc.net/").await.unwrap();
            assert_eq!(contest.title().await.unwrap(), "Baekjoon Online Judge");
        });

        driver.exit().await.unwrap();
    }
}