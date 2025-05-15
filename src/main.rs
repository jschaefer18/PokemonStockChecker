use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // Set up Chrome capabilities
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;

    // Connect to WebDriver (make sure chromedriver is running on port 9515)
    let driver = WebDriver::new("http://localhost:9515", caps).await?;




    // Navigate to a page
    driver.get("https://www.rust-lang.org").await?;

    // Get the page title
    let title = driver.title().await?;
    println!("Page title: {}", title);

    // Clean up
    driver.quit().await?;
    Ok(())
}
