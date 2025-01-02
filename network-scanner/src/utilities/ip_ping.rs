use futures::future::join_all;
use std::net::IpAddr;
use std::time::Duration;
use surge_ping::{Client, PingIdentifier, PingSequence};
use crate::utilities::ip_resolve::IpInfo;

async fn ping(dest: IpAddr, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let ip = tokio::net::lookup_host(format!("{}:0", dest))
        .await
        .expect("host lookup error")
        .next()
        .map(|val| val.ip())
        .unwrap();
    
    let payload = vec![0; 64];
    
    let mut pinger = client.pinger(ip, PingIdentifier(111)).await;

    pinger.timeout(Duration::from_secs(5));

    match pinger.ping(PingSequence(0), &payload).await {
        Ok((_packet, _rtt)) => {
            Ok(())
        }
        Err(e) => {
            Err(Box::new(e))
        }
    }
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

    let active_ips: Vec<IpInfo> = ping_results.into_iter().flatten().collect();  // `.flatten()` removes `None` values

    Ok(active_ips)
}