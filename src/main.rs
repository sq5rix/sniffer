use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::*;
use std::thread;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

// const MAX: u16 = 6555;
const MAX: u16 = 65535;

fn main() {
    let matches = App::new("sniffer")
        .author("author: tom")
        .about("sniffer snifs ports of an URL address")
        .after_help("Use with care...")
        .version("v0.1")
        .arg(
            Arg::with_name("verbose")
                .required(false)
                .help("verbose ports")
                .short("v"),
        )
        .arg(
            Arg::with_name("jobs")
                .default_value("4")
                .number_of_values(1)
                .takes_value(true)
                .help("number of threads/jobs")
                .short("j"),
        )
        .arg(
            Arg::with_name("ipaddr")
                // .multiple(true)
                .required(true)
                .help("IP addr to sniff"),
        )
        .get_matches();

    let addr = value_t!(matches, "ipaddr", IpAddr).expect("Provide IPv4 or IPv6 address ");
    let num_threads = value_t!(matches, "jobs", u16).expect("Provide number of threads");
    let is_verbose = matches.is_present("verbose");
    ();

    println!("Number of jobs: {}", num_threads);
    println!("IP address: {}", addr);

    let (tx, rx) = channel();

    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads, is_verbose);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("");
    if is_verbose {
        println!("Open ports:")
    }
    out.sort();
    for v in out {
        if is_verbose {
            println!("{} is open", v);
        } else {
            println!("{}", v);
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16, is_verbose: bool) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {
                if is_verbose {
                    println!("{}", port);
                }
            }
        }
        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}
