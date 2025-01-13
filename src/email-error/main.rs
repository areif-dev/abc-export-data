use clap::Parser;
use export_data::Config;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport, Transport};
use serde::Deserialize;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value_t = String::from("./config.json"))]
    config: String,
    message: String,
}

fn send_panic_email(config: &Config, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let email = lettre::Message::builder()
        .from(
            format!("<{}>", &config.notifier_email)
                .parse()
                .map_err(|_| lettre::error::Error::MissingFrom)?,
        )
        .to(format!("<{}>", &config.admin_email)
            .parse()
            .map_err(|_| lettre::error::Error::MissingTo)?)
        .subject("ABC Export Data Notification")
        .header(lettre::message::header::ContentType::TEXT_PLAIN)
        .body(message.to_string())?;
    let creds = Credentials::new(
        config.notifier_email.to_string(),
        config.notifier_passwd.to_string(),
    );
    let mailer = SmtpTransport::relay(&config.smtp_relay)?
        .credentials(creds)
        .port(config.smtp_port)
        .build();
    mailer.send(&email)?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();
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
    let config: Config = serde_json::from_reader(config_file).expect(&format!(
        "Config file is improperly formatted. Expected format: \n{:#?}\n",
        example_config
    ));
    send_panic_email(&config, &cli.message).expect("Failed to send email");
}
