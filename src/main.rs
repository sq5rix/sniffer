// use std::env;
// use std::net::IpAddr;
// use std::str::FromStr;
extern crate clap;
use clap::{App, Arg};

// struct Arguments {
//     flag: String,
//     ipaddr: IpAddr,
//     threads: u16,
// }

// impl Arguments{
//     fn new(args: &[String]) -> Result<Arguments, &'static str>{
//     }
// }

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
        .arg(Arg::with_name("URL").required(true).help("URL to sniff"))
        .get_matches();

    println!("{:?}", matches);
}
