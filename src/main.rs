use std::time::Duration;

use abc_uiautomation::{reports, UIElement};
use clap::Parser;
use export_data::Config;
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

    println!("Attempting to export Item data");
    reports::generate_report_11(&abcwin, "", "")
        .expect(&format!("Failed to generate 1-1 report from ABC"));

    println!("Success!");
}
