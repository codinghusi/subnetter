use std::fmt::{Display, Formatter};
use crate::ipv4::IPv4;
use crate::subnetmask::SubnetMask;

#[derive(Clone)]
pub struct SubnettedIP {
    pub ip: IPv4,
    pub mask: SubnetMask
}

impl SubnettedIP {
    pub fn next_network_ip(&self, n: u32) -> IPv4 {
        let bitmask = self.mask.raw();
        let adder = self.mask.ip_count();
        let raw = self.ip.raw();

        // println!("{:<10} {}", "prefix: ", self.mask.0);
        // println!("{:<10} {:#034b}", "bitmask: ", bitmask);
        // println!("{:<10} {:#034b}", "adder: ", adder);
        // println!("{:<10} {:#034b}", "raw: ", raw);

        IPv4::from_raw((raw & bitmask) + adder * n)
    }

    pub fn into_network_ip(self) -> Self {
        SubnettedIP {
            ip: self.network_ip(),
            mask: self.mask
        }
    }

    pub fn network_ip(&self) -> IPv4 {
        self.next_network_ip(0)
    }

    pub fn into_next_network_ip(self, n: u32) -> Self {
        SubnettedIP {
            ip: self.next_network_ip(n),
            mask: self.mask
        }
    }

    pub fn nth_ip(&self, n: u32) -> IPv4 {
        let subnetted_ip = self.next_network_ip(0);
        IPv4::from_raw(subnetted_ip.raw() + n)
    }

    pub fn first_host(&self) -> IPv4 {
        self.nth_ip(1)
    }

    pub fn broadcast_ip(&self) -> IPv4 {
        self.nth_ip(self.mask.ip_count())
    }

    pub fn last_host(&self) -> IPv4 {
        self.nth_ip(self.mask.ip_count() - 1)
    }
}


impl Display for SubnettedIP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("{}/{}", self.ip, self.mask.0).fmt(f)
    }
}
