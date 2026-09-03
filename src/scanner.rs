use futures::future;
use futures::stream::StreamExt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};
use tokio::io::{AsyncReadExt, AsyncWriteExt};


#[derive(Debug)]
pub struct ScanResult {
    pub port: u16,
    pub is_open: bool,
    pub banner: Option<String>
}

pub async fn read_with_timeout(stream: &mut TcpStream, timeout_ms: u64) -> Option<String> {
    let mut bytes: Vec<u8> = vec![0; 1024];
    let resp = timeout(
                    Duration::from_millis(timeout_ms),
                    stream.read(&mut bytes)).await;
    match resp {
            Ok(Ok(n)) => Some(String::from_utf8_lossy(&bytes[..n]).to_string()),
            Ok(Err(_)) => None,
            Err(_) => None
    }
}

pub async fn grab_banner(ip: Ipv4Addr, port: u16, timeout_ms: u64) -> Option<String> {
    let socket: SocketAddr = SocketAddr::new(IpAddr::V4(ip), port);

    let res: Result<Result<TcpStream, std::io::Error>, tokio::time::error::Elapsed> = timeout(
        Duration::from_millis(timeout_ms),
        TcpStream::connect(socket),
    )
    .await;

    match res {
        Ok(Ok(mut stream)) => {let awnser = read_with_timeout(&mut stream, timeout_ms).await;
                                            match awnser {
                                                None => {stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await.ok();
                                                        read_with_timeout(&mut stream, timeout_ms).await},
                                                _ => awnser
                                            }
                                        },
        _ => None,
    }
}

pub async fn scan_port(ip: Ipv4Addr, port: u16, timeout_ms: u64) -> ScanResult {
    let socket: SocketAddr = SocketAddr::new(IpAddr::V4(ip), port);

    let res: Result<Result<TcpStream, std::io::Error>, tokio::time::error::Elapsed> = timeout(
        Duration::from_millis(timeout_ms),
        TcpStream::connect(socket),
    )
    .await;

    match res {
        Ok(Ok(_)) => ScanResult {
            port,
            is_open: true,
            banner: grab_banner(ip, port, timeout_ms).await
        },
        Ok(Err(_)) => ScanResult {
            port,
            is_open: false,
            banner: None
        },
        Err(_) => ScanResult {
            port,
            is_open: false,
            banner: None
        },
    }
}

pub async fn scan_ports(ip: Ipv4Addr, start_port: u16, end_port: u16, timeout_ms: u64, concurrency: usize) -> Vec<ScanResult> {
    let mut ports: Vec<ScanResult> = futures::stream::iter(start_port..=end_port)
        .map(|port: u16| scan_port(ip, port, timeout_ms))
        .buffer_unordered(concurrency)
        .filter(|x: &ScanResult| future::ready(x.is_open == true))
        .collect::<Vec<ScanResult>>()
        .await;

    ports.sort_by(|a: &ScanResult, b: &ScanResult| a.port.cmp(&b.port));

    return ports;
}
