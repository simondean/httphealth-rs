use log::{error, info};
use reqwest::Client;
use std::env;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        info!("Usage: $ httphealth url [status_code]");
        info!("Example: $ httphealth https://example.org 200");
        process::exit(2);
    }

    let url = &args[1];
    let status_code = if args.len() > 2 {
        args[2].parse::<u16>().unwrap()
    } else {
        200
    };

    let client = Client::new();
    info!("Calling GET {}", url);
    let response = client.get(url).send().await?;
    let actual_status_code = response.status().as_u16();

    if actual_status_code == status_code {
        info!("OK: Expected status code {} returned", actual_status_code);
        process::exit(0);
    } else {
        error!(
            "ERR: Expected status code {}, actual status code {}",
            status_code, actual_status_code
        );
        process::exit(1);
    }
}
