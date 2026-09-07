mod args;
mod scanner;
mod ascii_art;

use args::Cli;
use clap::Parser;

use scanner::ScanResult;
use scanner::scan_ports;

use ascii_art::CRAB;

#[tokio::main]
async fn main() {
    let args : Cli = Cli::parse();
    assert!(args.start_port < args.end_port, "Error : Start port > End port !");
    let concurrency: usize = 200;
    let scan: Vec<ScanResult> = scan_ports(args.ip, args.start_port, args.end_port, args.timeout_ms, concurrency).await;  

    println!("{}", CRAB);
    println!("Scan report for {}", args.ip);
    println!("PORT    SERVICE");
    for port in &scan {
        match port.banner.as_deref() {
            None => println!("{:<8}Unknown", port.port),
            Some(s) if s.is_empty() => println!("{:<8}Unknown", port.port),
            Some(service) => println!("{:<8}{}", port.port, service.lines().next().unwrap_or("Unknown"))
        }
    } 
}
