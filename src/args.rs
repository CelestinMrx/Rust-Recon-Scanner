use crate::ascii_art::CRAB;

// use clap::{Args, Parser, Subcommand};
use clap::{Parser};

use std::net::Ipv4Addr;

#[derive(Parser, Debug)]
#[command(author, version, about = CRAB, long_about = None)]

pub struct Cli {
    /// IP cible
    #[arg(short, long)]
    pub ip: Ipv4Addr,
    /// Port de départ
    #[arg(short, long, default_value_t = 0)]
    pub start_port: u16,
    /// Port de fin
    #[arg(short, long, default_value_t = 6000)]
    pub end_port: u16,
    /// Timeout en millisecondes
    #[arg(short, long, default_value_t = 1000)]
    pub timeout_ms: u64,
}
