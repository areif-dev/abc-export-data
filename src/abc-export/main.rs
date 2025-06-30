use std::time::Duration;

use abc_uiautomation::{reports::generate_simple_report_with_skips, UIElement};
use clap::Parser;
use export_data::Config;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use serde::Deserialize;
use std::sync::{Arc, Once};
use std::{panic, process};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value_t = String::from("./config.json"))]
    config: String,
}

const MILLIS: u64 = 1;
const SECONDS: u64 = 1000 * MILLIS;
const MINUTES: u64 = 60 * SECONDS;
const TICKS: u64 = 500 * MILLIS;

fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let panic_message = match panic_info.payload().downcast_ref::<&str>() {
            Some(message) => *message,
            None => &panic_info.to_string(),
        };

        let location = if let Some(location) = panic_info.location() {
            format!("file '{}' at line {}", location.file(), location.line())
        } else {
            "unknown location".to_string()
        };

        let full_message = format!(
            "A panic occurred:\n\nMessage: {}\nLocation: {}",
            panic_message, location
        );

        if let Err(e) = process::Command::new("./email-error.exe")
            .arg(full_message)
            .output()
        {
            eprintln!("Failed to call email-error process: {:?}", e);
        }
    }));
}

fn export_file(abcwin: &UIElement, file: &str, data_path: &str, data_files: &[&str]) {
    println!("Generating report 7-10 for file {}", file);
    abc_uiautomation::reports::generate_report_710(&abcwin, file, false, "", "").unwrap();
    let mut ticks = 0 * TICKS;
    for file in data_files {
        println!("Waiting for file with path {}/{}", data_path, file);

        let path = &format!("{}/{}", data_path, file);
        while !abc_uiautomation::data_file_is_ready(path).expect(&format!(
            "Encountered an unexpected IO error while waiting for {}",
            path
        )) {
            if ticks >= 3 * MINUTES {
                panic!(
                    "Timeout. Waited for {} to load for greater than 3 minutes",
                    path
                );
            }

            std::thread::sleep(Duration::from_millis(1 * TICKS));
            ticks += 1 * TICKS;
        }
        println!("Waited for {} seconds", ticks as f64 / SECONDS as f64);
        // Wait an extra tick for ABC UI to catch up
        std::thread::sleep(Duration::from_millis(1 * TICKS));
        println!(
            "Moving data file from {}/{} to ./data/{}",
            data_path, file, file
        );
        std::fs::copy(path, &format!("./data/{}", file))
            .expect(&format!("Failed to move {}", file));
    }
}

fn main() {
    setup_panic_hook();
    let cli = Cli::parse();
    println!("Opening config file");
    let config_file = std::fs::File::open(&cli.config).expect(&format!(
        "Failed to open config file at {}. Does it exist?",
        cli.config
    ));
    let example_config = Config {
        abc_username: "String".to_string(),
        abc_password: "String".to_string(),
        notifier_email: "String".to_string(),
        notifier_passwd: "String".to_string(),
        smtp_relay: "String".to_string(),
        smtp_port: 465,
        admin_email: "String".to_string(),
    };
    println!("Parsing config file");
    let config: Config = serde_json::from_reader(config_file).expect(&format!(
        "Config file is improperly formatted. Expected format: \n{:#?}\n",
        example_config
    ));

    if !std::path::Path::new("./data").exists() {
        println!("Data directory is missing. Creating it now");
        std::fs::create_dir("./data").expect("Failed to create data directory");
    }
    println!("Data directory is present");
    println!("Looking for active ABC Client4 window");
    let abcwin =
        abc_uiautomation::ensure_abc().expect("Failed to find active ABC Client 4 instance");

    if let Some(popup) = abc_uiautomation::find_popup(&abcwin).unwrap_or(None) {
        panic!(
            "ABC has an unexpected popup after finding active ABC Client 4 instance: {:#?}",
            popup
        );
    }

    println!("Attempting to login to ABC");
    abc_uiautomation::login(&abcwin, &config.abc_username, &config.abc_password)
        .expect("Failed to login to ABC");
    println!("Attempting to export Item data");
    export_file(
        &abcwin,
        "I",
        "C:/ABC Software/Database Export/Company001/Data",
        &["item.data", "item_posted.data"],
    );
    println!("Attempting to export Customer data");
    export_file(
        &abcwin,
        "C",
        "C:/ABC Software/Database Export/Company001/Data",
        &["customer.data", "customer_posted.data"],
    );
    println!("Success!");
}
