use std::fmt::{Display, Formatter};
use crate::ipv4::IPv4;

#[derive(Clone)]
pub struct SubnetMask(pub u8);

impl SubnetMask {
    pub fn by_host_count(count: u32) -> Self {
        let host_bits = ((count + 2) as f32).log2().ceil() as u8;
        let network_bits = 32 - host_bits;
        SubnetMask(network_bits)
    }

    pub fn raw(&self) -> u32 {
        (2u32.pow((32 - self.0) as u32) - 1) ^ u32::MAX
    }

    pub fn ip(&self) -> IPv4 {
        IPv4::from_raw(self.raw())
    }

    pub fn ip_count(&self) -> u32 {
        2u32.pow((32 - self.0) as u32)
    }

    pub fn network_count(&self) -> u32 {
        2u32.pow(self.0 as u32)
    }

    pub fn host_count(&self) -> u32 {
        self.ip_count() - 2
    }
}

impl Display for SubnetMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("{}", self.ip()).fmt(f)
    }
}
