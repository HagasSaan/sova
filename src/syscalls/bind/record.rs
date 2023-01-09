use crate::syscalls::bind::sockaddr_in::SockaddrIn;

#[derive(Debug)]
pub struct Record {
    pub sockfd: i32,
    pub addr: SockaddrIn,
    pub addrlen: u32,
}
