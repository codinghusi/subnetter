
// use std::io;
// use regex::{Error, Regex};
//
// fn parse_ip(raw: &str) -> Result<(), String> {
//     let re = Regex::new(r"(\d{1,3}).(\d{1,3}).(\d{1,3}).(\d{1,3})\s*/(\d{1,2})")
//         .or_err(format!("Die IP wurde nicht im korrekten Format eingegeben."))?;
//
//
// }
//
// fn main() {
//     println!("Gebe die deinen IP-Adressbereich an (Beispiel: 192.168.0.0/16)");
//     print!("IP> ");
//     let mut ip_raw = String::new();
//     io::stdin().read_line(&mut ip_raw).unwrap();
//
//
//
// }