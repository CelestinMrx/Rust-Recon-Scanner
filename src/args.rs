use clap::{Args, Parser, Subcommand};

use std::net::Ipv4Addr;

#[derive(Parser, Debug)]
#[command(author, version, about ="
                         _,,--.._  
                        /. ` ` .  `.
                        )|       `  `.
           .           / |         `  `
            `.        / /            ` `
             `.`.    / /              ` `
               `.`.'' /                ' :
                <','/'`                . ;
               ,-'.-    `             , /
           _.-',-^`       `      _.-----
     /`==::.,-'     `       ` ,-'
    / /               `     .;
    | |..               ` .,' `.
    | ':`....---.       ,'`'.   `.
     .`:.:.:.:.:-..    /     `.   `.
      .`ccoccoccoc'``./        `.   `.
       `.`CQCCQCCCQCC/           `.   `.
         `.`8O8O8O8O8(             `.   `.
           `.`_-_@-@_-;              `. .'''.
                ''''                   :,' ,--'
                                        `.` _,--
                 A                        `.  _,',.
                (@)                         `. .-' `_
                                              `. ,-^.`.
                   A                            `. - _.-.
                  (@)                             `.', ,'-
                                                    `. _,-`__
                                                      `. _-,`|
                                                        |,_-`|
                                                        '----'
 Scanner de reconnaissance réseau test
 ", long_about = None)]

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
    /// Timeout
    #[arg(short, long, default_value_t = 10)]
    pub timeout: u64,
}
