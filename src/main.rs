mod network;
mod duckdns;

use std::time::Duration;
use anyhow::{anyhow, Error};
use tokio::time::sleep;
use clap::Parser;
use log::{error, LevelFilter, warn};
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The subdomain ({domain}.duckdns.org) to update
    #[arg(long)]
    domain: String,

    /// The DuckDNS token
    #[arg(long)]
    token: String,
}

fn print_error(error: Error, previous_error: &mut Option<Error>) {
    if previous_error.is_none() || previous_error.as_ref().unwrap().to_string() != error.to_string() {
        error!("{error}");
        previous_error.replace(error);
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Stdout, ColorChoice::Auto)
        .expect("Failed to set up logger");

    let ddns_client = duckdns::Client::new(args.domain, args.token);

    let mut previous_ipv4 = None;
    let mut previous_ipv6 = None;
    let mut previous_error: Option<Error> = None;
    loop {
        match network::get_addresses().await {
            Ok((ipv4, ipv6)) => {
                if ipv4 != previous_ipv4 || ipv6 != previous_ipv6 {
                    warn!("Updating to {ipv4:?}, {ipv6:?}");
                    if let Err(e) = ddns_client.update(ipv4, ipv6).await {
                        let error = anyhow!("Failed to update DuckDNS: {}", e);
                        print_error(error, &mut previous_error);
                    } else {
                        previous_ipv4 = ipv4;
                        previous_ipv6 = ipv6;
                        previous_error = None;
                    }
                }
            }
            Err(e) => {
                let error = anyhow!("Failed to get address: {}", e);
                print_error(error, &mut previous_error);
            }
        }

        sleep(Duration::from_millis(500)).await;
    }
}
