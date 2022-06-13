use std::fmt::{Display, Formatter};
use crate::subnetmask::SubnetMask;
use crate::subnetted_ip::SubnettedIP;

#[derive(Clone)]
pub struct IPv4(pub u8, pub u8, pub u8, pub u8);

impl IPv4 {
    pub fn raw(&self) -> u32 {
        (self.0 as u32) << 24 |
            (self.1 as u32) << 16 |
            (self.2 as u32) << 8 |
            (self.3 as u32)
    }

    pub fn from_raw(raw: u32) -> Self {
        IPv4(
            (raw >> 24 & 0xFF) as u8,
            (raw >> 16 & 0xFF) as u8,
            (raw >> 8  & 0xFF) as u8,
            (raw >> 0  & 0xFF) as u8,
        )
    }

    pub fn with(self, mask: SubnetMask) -> SubnettedIP {
        SubnettedIP {
            ip: self,
            mask
        }
    }
}

impl Display for IPv4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("{}.{}.{}.{}", self.0, self.1, self.2, self.3).fmt(f)
    }
}