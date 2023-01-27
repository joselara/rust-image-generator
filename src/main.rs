use std::fs;
use std::path::PathBuf;

use futures::StreamExt;

use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::page::CaptureScreenshotFormat,
    handler::viewport::Viewport,
    page::ScreenshotParams,
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .window_size(1400, 900)
            .viewport(Viewport {
                width: 1400,
                height: 900,
                device_scale_factor: None,
                emulating_mobile: false,
                is_landscape: true,
                has_touch: false,
            })
            .args([
                "--allow-file-access-from-files",
                "--enable-local-file-accesses",
            ])
            .build()?,
    )
    .await?;

    let handle = async_std::task::spawn(async move {
        loop {
            let _event = handler.next().await.unwrap();
        }
    });

    let html = PathBuf::from("sample/index.html");
    let filepath = fs::canonicalize(&html).unwrap();

    let page = browser
        .new_page(format!("file://{}", filepath.display()))
        .await?;

    let element = page.find_element("body > div").await?;

    // save the page as pdf
    page.wait_for_navigation()
        .await?
        .save_screenshot(
            ScreenshotParams::builder()
                .format(CaptureScreenshotFormat::Png)
                .full_page(false)
                .build(),
            "media-content.png",
        )
        .await?;

    handle.await;
    Ok(())
}
