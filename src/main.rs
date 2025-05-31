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
    sequence_number: u16,
}

fn create_icmp_packet(seq: u16) -> Vec<u8> {
    let mut packet = vec![0u8; 8]; // ICMP header 8 bytes

    let header = IcmpHeader{
        icmp_type: ICMP_ECHO_REQUEST,
        icmp_code:0,
        checksum:0,
        identifier: 0x1234,
        sequence_number: seq,
    };

    unsafe{
        let header_bytes = std::slice::from_raw_parts(
            &header as *const IcmpHeader as *const u8,
            mem::size_of::<IcmpHeader>()
        );
        packet[..8].copy_from_slice(header_bytes);
    }

    let cs = checksum(&packet);
    packet[2] = (cs >> 8) as u8;
    packet[3] = (cs & 0xFF) as u8;

    packet 
}

fn main() {
    println!("Hello, world!");
}
