use std::{env, net::IpAddr};

struct Arguements{
    flag: String,
    ipaddr: IpAddr,
    threads: u16    
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let program = args[0].clone();

}
