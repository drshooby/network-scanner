use futures::future::join_all;
use std::net::IpAddr;
use std::time::Duration;
use surge_ping::{Client, PingIdentifier, PingSequence};
use rand::random;
use tokio::time::interval;
use crate::utilities::ip_resolve::IpInfo;

async fn ping(dest: IpAddr, client: &Client) -> Result<(), Box<dyn std::error::Error>> {

    let payload = vec![0; 56];

    let mut pinger = client.pinger(dest, PingIdentifier(random())).await;

    pinger.timeout(Duration::from_secs(1));
    let mut interval = interval(Duration::from_secs(1));
    for i in 0..5 {
        interval.tick().await;
        match pinger.ping(PingSequence(i), &payload).await {
            Ok((_packet, _rtt)) => {
                return Ok(());
            }
            Err(_) => {},
        }
    }
    Err("Ping timed out".into())
}

pub(crate) async fn check_active_ips(ips: Vec<IpInfo>, client: Client) -> Result<Vec<IpInfo>, Box<dyn std::error::Error>> {
    let ping_results = join_all(ips.into_iter().map(|ip_info| {
        let ip = ip_info.ip;
        let client = client.clone();
        async move {
            match ping(ip, &client).await {
                Ok(_) => Some(ip_info),
                Err(_) => None,
            }
        }
    })).await;

    let active_ips: Vec<IpInfo> = ping_results
        .into_iter()
        .flatten()
        .collect();

    Ok(active_ips)
}