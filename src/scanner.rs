use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use futures::stream::StreamExt;
use futures::future;

#[derive(Debug)]
pub struct ScanResult {
    pub port: u16,
    pub is_open: bool,
}

pub async fn scan_port(ip: Ipv4Addr, port: u16, timeout_ms: u64) -> ScanResult {
    let socket = SocketAddr::new(IpAddr::V4(ip), port);
    
    let res = timeout(Duration::from_millis(timeout_ms), TcpStream::connect(socket)).await;

    match res {
        Ok(Ok(_)) => ScanResult { port, is_open: true },
        Ok(Err(_)) => ScanResult { port, is_open: false },
        Err(_) => ScanResult { port, is_open: false },
    }
}

pub async fn scan_ports(ip: Ipv4Addr, start_port: u16, end_port: u16, timeout_ms: u64, concurrency: usize) -> Vec<ScanResult> {
    let mut ports = futures::stream::iter(start_port..end_port)
    .map(|port: u16| scan_port(ip, port, timeout_ms))
    .buffer_unordered(concurrency)
    .filter(|x| future::ready(x.is_open == true))
    .collect::<Vec<ScanResult>>()
    .await;

    ports.sort_by(|a, b| a.port.cmp(&b.port));
    
    return ports;
}