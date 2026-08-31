mod args;
mod scanner;

use args::Cli;
use clap::Parser;

use scanner::ScanResult;
use scanner::scan_port;

#[tokio::main]
async fn main() {
    let args : Cli = Cli::parse();
    let scan: ScanResult = scan_port(args.ip, args.start_port, args.timeout_ms).await;    
    println!("{:?}", scan);
}
