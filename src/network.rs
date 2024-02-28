use std::net::{Ipv4Addr, Ipv6Addr};
use anyhow::{anyhow, Error};
use itertools::Itertools;
use net_route::Handle;
use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig};

pub async fn get_addresses(if_name: &Option<String>) -> Result<(Option<Ipv4Addr>, Option<Ipv6Addr>), Error> {
    let target_interface = if let Some(if_name) = if_name {
        NetworkInterface::show()?
            .into_iter()
            .find_or_first(|i| i.name == *if_name)
            .ok_or(anyhow!("Could not find specified interface"))?
    } else {
        let interface_index = Handle::new()?
            .list()
            .await?
            .into_iter()
            .filter(|r| r.prefix == 0)
            .filter(|r| r.destination == Ipv4Addr::UNSPECIFIED || r.destination == Ipv6Addr::UNSPECIFIED)
            .sorted_by_key(|r| r.metric.ok_or(u32::MAX))
            .filter_map(|r| r.ifindex)
            .next()
            .ok_or(anyhow!("No default route found"))?;

        NetworkInterface::show()?
            .into_iter()
            .find_or_first(|i| i.index == interface_index)
            .ok_or(anyhow!("Could not find interface for default route"))?
    };

    let mut found_v4 = None;
    let mut found_v6 = None;
    for addr in target_interface.addr {
        match addr {
            Addr::V4(a) => {
                found_v4 = Some(a.ip);
            }
            Addr::V6(a) => {
                if a.ip.octets()[0] >> 4 == 2 { // janky hack m8
                    found_v6 = Some(a.ip);
                }
            }
        }
    }
    Ok((found_v4, found_v6))
}
