mod args;
mod scanner;

use args::Cli;
use clap::Parser;

use scanner::ScanResult;
// use scanner::scan_port;
use scanner::scan_ports;


#[tokio::main]
async fn main() {
    let args : Cli = Cli::parse();
    let concurrency: usize = 200;
    let scan: Vec<ScanResult> = scan_ports(args.ip, args.start_port, args.end_port, args.timeout_ms, concurrency).await;   
    println!(r"                                ,-.
                               ('O_)
                              / `-/
                             /-. /
                            /   )
                           /   /  
              _           /-. /
             (_)'-._     /   )
               '-._ '-'''( )/    
                   '-/'-._' `. 
                    /     '-.'._
                   /\\       /-._'-._
    _,---...__    /  ) _,-'/    '-(_)
___<__(|) _   ''-/  / /   /
 '  `----' '-.   \\/ /   /
               )  ] /   /
       ____..-'   //   /                       )
   ,-''      __.,'/   /   ___                 /,
  /    ,--''/  / /   /,-''   '''-.          ,'/
 [    (    /  / /   /  ,.---,_   `._   _,-','
 \\    `-./  / /   /  /       `-._  ''' ,-'
   `-._  /  / /   /_,'            ''--'
       '/  / /   /'         
       /  / /   /
      /  / /   /  
     /  |,'   /  
    :   /    /
    [  /   ,'   
    | /  ,'
    |/,-'
    P'
    ");
    println!("Scan report for {}", args.ip);
    println!("PORT   SERVICE");
    for port in &scan {
        println!("{}   Unknown", port.port);
    } 
}
