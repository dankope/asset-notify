use std::path::Path;

use thirtyfour::prelude::*;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let args: Vec<String> = std::env::args().collect();
    env_logger::init();

    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--headless")?;
    caps.add_chrome_arg("--no-sandbox")?;

    // Set chrome binary
    if let Some(chrome_bin) = args.get(1) {
        caps.set_binary(&chrome_bin)?;
    }

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://asset.party/~get").await?;

    let mut slots = 0;

    // Kek.
    loop {
        // Refresh if we are blocked by website update.
        if driver.source().await?.contains("please refresh page") {
            driver.refresh().await?;
        }

        if slots > 0 {
            // Last loop there were some claimable buttons!
            log::info!("Last tick had {} slots available!", slots);

            let body = driver.find(By::ClassName("squash card")).await?;
            body.screenshot(Path::new(&format!("captures/{}", slots).to_string()))
                .await?;

            // Notify and sleep for 2 seconds (then check if there are still claimable buttons).

            sleep(std::time::Duration::from_secs(2)).await;
        }

        let buttons = driver.find_all(By::ClassName("button")).await?;
        for btn in buttons {
            if btn.is_enabled().await? || btn.css_value("opacity").await? == "1.0" {
                log::debug!(
                    "Possible button: {} with class: {:?}",
                    btn,
                    btn.class_name().await?
                );

                // TODO: Login to your steam account then click the claimable button!

                slots += 1;
            }
        }
    }
}
