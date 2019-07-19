// use std::env;
use std::net::IpAddr;
// use std::str::FromStr;
#[macro_use]
extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("sniffer")
        .author("author: tom")
        .about("sniffer snifs ports of an URL address")
        .after_help("Use with care...")
        .version("v0.1")
        .arg(
            Arg::with_name("threads")
                .default_value("4")
                .number_of_values(1)
                .takes_value(true)
                .help("number of threads")
                .short("t"),
        )
        .arg(
            Arg::with_name("ipaddr")
                // .multiple(true)
                .required(true)
                .help("IP addr to sniff"),
        )
        .get_matches();

    let ipaddr = value_t!(matches, "ipaddr", IpAddr).expect("Provide IPv4 or IPv6 address ");
    let threads = value_t!(matches, "threads", u16).expect("Provide number of threads");

    println!("{:?}", ipaddr);
    println!("{:?}", threads);
}
