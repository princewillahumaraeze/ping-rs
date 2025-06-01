use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::os::unix::io::AsRawFd;
use std::time::Instant;


#[repr(packed)]
struct IcmpEcho {
    icmp_type: u8,
    icmp_code: u8,
    checksum: u16,
    identifier: u16,
    sequence_number: u16,
    payload: Vec<u8>,
}


fn checksum(data: &[u8]) -> u16 {
    let mut sum =  0u32;
    let mut chunks = data.chunks_exact(2);


    for chunk in &mut chunks {
        let value = u16::from_be_bytes([chunk[0], chunk[1]]);
        sum += value as u32;
    }

    if let Some(&last_byte) = chunks.remainder().first(){
        sum += (last_byte as u32) << 8;
    }

    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    !(sum as u16)
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

    
    packet 
}

fn main() {
    println!("Hello, world!");
}
