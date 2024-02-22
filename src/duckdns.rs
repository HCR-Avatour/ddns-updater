use std::net::{Ipv4Addr, Ipv6Addr};
use anyhow::{anyhow, Error};
use log::{info};
use reqwest::StatusCode;

pub struct Client {
    domain: String,
    token: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new<S: Into<String>>(domain: S, token: S) -> Client {
        return Client {
            domain: domain.into(),
            token: token.into(),
            client: reqwest::Client::new(),
        }
    }

    async fn request(&self, url: String) -> Result<(), Error> {
        info!("Making request to {url}");
        let response = self.client.get(url).send().await?;
        let status = response.status();
        let text = response.text().await?;
        if status != StatusCode::OK {
            Err(anyhow!("DuckDNS returned status code {status}"))
        } else if text != "OK" {
            Err(anyhow!("DuckDNS didn't say OK: {text}"))
        } else {
            info!("Response: {text}");
            Ok(())
        }
    }

    pub async fn update(&self, ipv4: Option<Ipv4Addr>, ipv6: Option<Ipv6Addr>) -> Result<(), Error> {
        let ipv4str = ipv4.map(|a| a.to_string()).unwrap_or(String::new());
        let ipv6str = ipv6.map(|a| a.to_string()).unwrap_or(String::new());
        self.request(format!("https://www.duckdns.org/update?domains={}&token={}&clear=true", self.domain, self.token)).await?;
        self.request(format!("https://www.duckdns.org/update?domains={}&token={}&ip={ipv4str}&ipv6={ipv6str}", self.domain, self.token)).await
    }
}
