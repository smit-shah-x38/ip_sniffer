use std::{env, net::{IpAddr, TcpStream}, process, sync::mpsc::{Sender, channel}, thread, io::{self, Write}};
use std::str::FromStr;

const MAX: u16 = 65535;

struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16    
}

impl Arguments{
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguements");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments { flag: String::from(""), ipaddr, threads: 4 });            
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!("This is supposed to be a help menu.");
                return Err("Help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Invalid IP Address, must be IPv4 or IPv6.") 
                };
                
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Invalid thread count.")                    
                };

                return Ok(Arguments {
                    threads,
                    flag,
                    ipaddr,
                });

            } else {
                return Err("Invalid Syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16){
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect(format!("{}:{}", addr, start_port)){
            Ok(_) => {
                print!("_");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguements = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("Help"){ 
                process::exit(0);
            } else { 
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
    });

    let addr = arguements.ipaddr;
    let num_threads = arguements.threads;
    let (tx, rx) = channel();

    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
