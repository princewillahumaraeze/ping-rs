use std::io::{self, Write};
use std::mem;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::os::unix::io::AsRawFd;
use std::time::{Duration};
use std::net::UdpSocket;

const ICMP_ECHO_REQUEST: u8 = 8;
const ICMP_ECHO_REPLY: u8 = 0;

#[repr(packed)]
struct IcmpHeader{
    icmp_type: u8,
    icmp_code: u8,
    checksum: u16,
    identifier: u16,
    sequence_header: u16,
}

fn main() {
    println!("Hello, world!");
}
