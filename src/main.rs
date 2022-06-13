use std::cmp::Reverse;
use std::fmt::{Debug, Display, Formatter};

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

fn main() {
    // Specific
    let scope = SubnettedIP {
        ip: IPv4(86,80,0,0),
        mask: SubnetMask(16)
    }.into_network_ip();

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
    println!(
        "{: <15} | {: <15} | {: <15} | {: <15} | {: <15}",
        "Netzname", "Netzwerkadresse", "Subnetzmaske", "Erste IP", "Letzte IP"
    );

    println!("{}", "=".repeat(17 * 5 + 4));

    let mut current_ip = scope.clone();
    for network in networks {
        let snm = SubnetMask::by_host_count(network.host_count);
        current_ip.mask = snm;
        println!(
            "{: <15} | {: <15} | {: <15} | {: <15} | {: <15}",
            network.name, current_ip.network_ip(), current_ip.mask, current_ip.first_host(), current_ip.last_host()
        );

        // println!("{}", current_ip);

        // println!("{:b}", snm.raw());
        // println!("0{:b}", current_ip.ip.raw());
        // println!("0{:b}", snm.ip_count());

        current_ip = current_ip.into_next_network_ip(1);
    }
}