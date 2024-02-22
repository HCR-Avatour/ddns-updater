use std::net::{Ipv4Addr, Ipv6Addr};
use anyhow::{anyhow, Error};
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

    pub async fn update(&self, ipv4: Option<Ipv4Addr>, ipv6: Option<Ipv6Addr>) -> Result<(), Error> {
        let ipv4str = ipv4.map(|a| a.to_string()).unwrap_or(String::new());
        let ipv6str = ipv6.map(|a| a.to_string()).unwrap_or(String::new());
        let url = format!("https://www.duckdns.org/update?domains={}&token={}&ipv4={ipv4str}&ipv6={ipv6str}", self.domain, self.token);
        let status = self.client.get(url)
            .send()
            .await?
            .status();
        if status != StatusCode::OK {
            Err(anyhow!("DuckDNS returned status code {status}"))
        } else {
            Ok(())
        }
    }
}
