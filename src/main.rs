use std::cmp::Reverse;
use std::fmt::{Debug, Display, Formatter};
use clap::Parser;
use regex::Regex;

mod ipv4;
mod subnetmask;
mod subnetted_ip;

use crate::ipv4::*;
use crate::subnetmask::*;
use crate::subnetted_ip::*;

struct NetworkRequirement {
    pub name: String,
    pub host_count: u32
}

fn get_num(name: &str) -> u8 {
    let mut chars = name.chars();
    let first = chars.next().unwrap() as u8;
    let second = chars.next().unwrap() as u8;
    (first + second) / 2
}

fn get_ip(prefix: u8, name: (String, String)) -> SubnettedIP {
    SubnettedIP {
        ip: IPv4(get_num(&name.0), get_num(&name.1), 0, 0),
        mask: SubnetMask(prefix)
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Eingabe des Vornamens
    #[clap(short, long, value_parser)]
    vorname: String,

    /// Eingabe des Nachnamens
    #[clap(short, long, value_parser)]
    nachname: String,

    /// Ausgabe in CSV-Format
    #[clap(long, value_parser, default_value_t = false)]
    csv: bool,

    /// Auswahl des Prefixes (Wert von 0 bis 32)
    #[clap(short, long, value_parser, default_value_t = 12)]
    prefix: u8,
}

fn main() {
    // let prefix = 12;
    // let name = (String::from("Vorname"), String::from("Nachname"));
    let args = Args::parse();

    let prefix = args.prefix;
    let name = (args.vorname, args.nachname);
    let with_csv = args.csv;

    if prefix > 32 {
        panic!("Der Wert von Prefix muss zwischen 0 und 32 sein.");
    }

    // Specific
    let scope = get_ip(prefix, name).into_network_ip();

    let mut networks = vec![
        ("Aachen 1", 20_000),
        ("Aachen 2", 3_650),
        ("Aachen 3 und 4", 1_000 + 1_250),
        ("Aachen 5", 1_000),
        ("Aachen 6", 50),
        ("Aachen 7", 600),
        ("Madrid", 1_500),
        ("Stockholm 1", 14_000),
        ("Stockholm 2", 2_000),
        ("Amsterdam", 2_800),
        ("Moskau 1", 200),
        ("Moskau 2", 150),
        ("VN1", 2), ("VN2", 2), ("VN3", 2),
        ("VN4", 2), ("VN5", 2), ("VN6", 2),
        ("VN7", 2), ("VN8", 2), ("VN9", 2),
        ("Webserver", 2),
        ("Internet Prov.", 2),
    ].into_iter()
        .map(|(name, host_count)| NetworkRequirement {
            name: String::from(name),
            host_count
        })
        .collect::<Vec<_>>();

    // Code
    networks.sort_by(|network2, network1|
        network1.host_count.cmp(&network2.host_count)
    );

    println!("--------------------------------------------------");
    println!(" Netzwerk mit IP-Bereich {} ", scope);
    println!("--------------------------------------------------");
    println!();

    if with_csv {
        println!(
            "{},{},{},{},{},{},{}",
            "Netzname", "Anzahl der Rechner", "Netzwerkadresse", "Subnetzmaske", "Erste IP", "Letzte IP", "Broadcastadresse"
        )
    } else {
        println!(
            "{: <15} | {: <6} | {: <15} | {: <15} | {: <15} | {: <15} | {: <15}",
            "Netzname", "#Hosts", "Netzwerkadresse", "Subnetzmaske", "Erste IP", "Letzte IP", "Broadcastadresse"
        );
    }


    if !with_csv {
        println!("{}", "=".repeat(17 * 6 + 5 + 6));
    }

    let mut current_ip = scope.clone();
    for network in networks {
        let snm = SubnetMask::by_host_count(network.host_count);
        current_ip.mask = snm;

        if with_csv {
            println!(
                "{},{},{},{},{},{}",
                network.name, network.host_count, current_ip.network_ip(), current_ip.mask, current_ip.first_host(), current_ip.last_host()
            );
        } else {
            println!(
                "{: <15} | {: >6} | {: <15} | {: <15} | {: <15} | {: <15} | {: <15}",
                network.name, network.host_count, current_ip.network_ip(), current_ip.mask, current_ip.first_host(), current_ip.last_host(), current_ip.broadcast_ip()
            );
        }


        current_ip = current_ip.into_next_network_ip(1);
    }
}