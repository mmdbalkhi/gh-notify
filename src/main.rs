use std::{path, thread, time};

use api::request;
use utils::{get_api, notify, write_api, PATH};

mod api;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schedule = 60 * 5 * 1000; // 5m
    let five_minutes = time::Duration::from_millis(schedule); // TODO: Customizable

    let path_is_exists = path::Path::new(PATH).exists();
    if !path_is_exists {
        write_api()?;
    }
    let api_key: String = get_api();

    loop {
        let response = request(api_key.as_str()).await;
        if response.as_array().unwrap().is_empty() {
            thread::sleep(five_minutes);
            continue;
        } else {
            let summary = format!(
                "{} - {} - {}",
                response[0]["repository"]["name"],
                response[0]["subject"]["type"],
                response[0]["subject"]["title"],
            );
            let body = format!(
                "your have an {} from {}",
                response[0]["subject"]["type"],
                response[0]["subject"]["url"]
                    .as_str()
                    .unwrap()
                    .replace("api.", "")
                    .replace("repos/", ""),
            );
            notify(summary.as_str(), body.as_str())?;
        }
        thread::sleep(five_minutes);
    }
}
