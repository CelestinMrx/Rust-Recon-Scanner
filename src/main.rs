mod args;
mod scanner;
mod ascii_art;

use args::Cli;
use clap::Parser;

use scanner::ScanResult;
use scanner::scan_ports;
//use scanner::grab_banner;

use ascii_art::SWORD;

#[tokio::main]
async fn main() {
    let args : Cli = Cli::parse();
    let concurrency: usize = 200;
    let scan: Vec<ScanResult> = scan_ports(args.ip, args.start_port, args.end_port, args.timeout_ms, concurrency).await;   
    println!("{}", SWORD);
    println!("Scan report for {}", args.ip);
    println!("PORT   SERVICE");
    for port in &scan {
        println!("{}   {:?}", port.port, port.banner);
        println!("");
    } 
}
