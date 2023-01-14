use libc::sockaddr_in;
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: Ipv4Addr,
    pub sin_zero: [u8; 8],
}

impl SockaddrIn {
    pub fn new(addr: sockaddr_in) -> Self {
        SockaddrIn {
            sin_family: addr.sin_family,
            sin_port: u16::from_be(addr.sin_port),
            sin_addr: Ipv4Addr::from(addr.sin_addr.s_addr),
            sin_zero: addr.sin_zero,
        }
    }
}
