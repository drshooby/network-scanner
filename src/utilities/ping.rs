use crate::utilities::resolve::IpInfo;
use dns_lookup::lookup_addr;
use futures::future::join_all;
use rand::random;
use std::net::IpAddr;
use std::time::Duration;
use surge_ping::{Client, PingIdentifier, PingSequence};
use tokio::time::interval;

pub struct Address {
    pub ip: IpAddr,
    pub hostname: String,
}

async fn ping(dest: IpAddr, client: &Client) -> Option<Address> {
    let payload = vec![0; 56];

    let mut pinger = client.pinger(dest, PingIdentifier(random())).await;
    pinger.timeout(Duration::from_secs(1));

    let mut interval = interval(Duration::from_secs(1));
    for i in 0..5 {
        interval.tick().await;
        if let Ok((_packet, _rtt)) = pinger.ping(PingSequence(i), &payload).await {
            let hostname = resolve_hostname(dest).await;
            return Some(Address {
                ip: dest,
                hostname,
            });
        }
    }
    None
}

async fn resolve_hostname(ip: IpAddr) -> String {
    match tokio::task::spawn_blocking(move || lookup_addr(&ip)).await {
        Ok(Ok(name)) => name,
        _ => String::from("N/A")
    }
}

pub(crate) async fn check_active_ips<I>(ip_range_it: I, client: Client) -> Result<Vec<Address>, Box<dyn std::error::Error>> where I: Iterator<Item = IpInfo> {
    let ping_results = join_all(ip_range_it.map(|ip_info| {
        let ip = ip_info.ip;
        let client = client.clone();
        async move {
            ping(ip, &client).await
        }
    })).await;

    let active_ips: Vec<Address> = ping_results
        .into_iter()
        .flatten()
        .collect();

    Ok(active_ips)
}