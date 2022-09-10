use std::{fs, io};

// use dirs::home_dir;
use notify_rust::Notification;
use serde_derive::{Deserialize, Serialize};

pub const PATH: &str = ".ghnotifyrc"; // TODO: store in HOME

#[derive(Serialize)]
struct Config {
    apikey: String,
}

#[derive(Deserialize)]
struct Key {
    apikey: String,
}

pub fn notify(summary: &str, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    //! ## notify
    //!
    //! this Function just Create a notification with msg
    //!
    Notification::new()
        .summary(summary)
        .body(msg)
        .icon("github")
        .appname("gh-notify")
        .show()?; // TODO: open github-notif on webbrowser when clicked
    Ok(())
}

#[warn(clippy::single_char_pattern)]
pub fn write_api() -> Result<(), Box<dyn std::error::Error>> {
    //! ## write_api
    //! this Function Get api-key from user and write this on a toml file
    //!
    //! - filepath: `$HOME/.ghnotifyrc
    //!

    println!("api-key not found. Enter your api-key(with *notification* permission)"); // FIX: use pretty stdin
    let mut api_key = String::new();
    io::stdin()
        .read_line(&mut api_key)
        .expect("Faild to read input");

    if !api_key.starts_with("ghp_") {
        panic!("api-key might be incorrect.");
    }

    let body = Config { apikey: api_key };
    let config = toml::to_string(&body).unwrap();

    fs::write(PATH, config).expect("Faild to write file");
    Ok(())
}

pub fn get_api() -> String {
    //! ## get_api
    //! this function get api-key from toml file
    //!

    let body = fs::read_to_string(PATH).expect("Faild to read file");
    let config: Key = toml::from_str(&body).unwrap();

    config.apikey.replace("\n", "")
}
