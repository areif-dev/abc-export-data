use std::time::Duration;

use abc_uiautomation::UIElement;
use clap::Parser;
use serde::Deserialize;

const MILLIS: u64 = 1;
const SECONDS: u64 = 1000 * MILLIS;
const MINUTES: u64 = 60 * SECONDS;
const TICKS: u64 = 500 * MILLIS;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value_t = String::from("./config.json"))]
    config: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    abc_username: String,
    abc_password: String,
}

fn export_file(abcwin: UIElement, file: &str, data_path: &str, data_files: &[&str]) -> UIElement {
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
        std::fs::rename(path, &format!("./data/{}", file))
            .expect(&format!("Failed to move {}", file));
    }

    abcwin
}

fn main() {
    if !std::path::Path::new("./data").exists() {
        println!("Data directory is missing. Creating it now");
        std::fs::create_dir("./data").expect("Failed to create data directory");
    }
    println!("Data directory is present");
    let cli = Cli::parse();
    println!("Reading config file at {}", &cli.config);
    let config_file = std::fs::File::open(&cli.config).expect(&format!(
        "Failed to open config file at {}. Does it exist?",
        cli.config
    ));
    let example_config = Config {
        abc_username: "String".to_string(),
        abc_password: "String".to_string(),
    };
    let config: Config = serde_json::from_reader(config_file).expect(&format!(
        "Config file is improperly formatted. Expected format: \n{:#?}\n",
        example_config
    ));

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
    let abcwin = export_file(
        abcwin,
        "I",
        "C:/ABC Software/Database Export/Company001/Data",
        &["item.data", "item_posted.data"],
    );
    println!("Attempting to export Customer data");
    let abcwin = export_file(
        abcwin,
        "C",
        "C:/ABC Software/Database Export/Company001/Data",
        &["customer.data", "customer_posted.data"],
    );
    println!("Success!");
}
