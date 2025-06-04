use std::io;
use std::net::{IpAddr, UdpSocket};
use std::os::unix::io::AsRawFd;
use std::time::Instant;


struct IcmpEcho {
    icmp_type: u8,
    icmp_code: u8,
    checksum: u16,
    identifier: u16,
    sequence_number: u16,
    payload: Vec<u8>,
}

impl IcmpEcho{
    fn new(identifier: u16, sequence_number:u16, payload: Vec<u8>) -> Self{
        IcmpEcho { 
            icmp_type: 8,
            icmp_code: 0,
            checksum: 0,
            identifier,
            sequence_number,
            payload,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let payload_clone = self.payload.clone();
        let packet_payload_len = 8 + payload_clone.len();
        let mut packet = Vec::with_capacity(packet_payload_len);

        // Icmp Header
        packet.push(self.icmp_type);                        //Byte 0: Icmp Type
        packet.push(self.icmp_code);                        //Byte 1: Icmp Code
        packet.push(0);                                     //Byte 2: Checksum placeholder(high byte)
        packet.push(0);                                     //Byte 3: Checksum placeholder(low byte)
        packet.push((self.identifier >> 8) as u8);          //Byte 4: Identifier(high byte)
        packet.push((self.identifier & 0xFF) as u8);        //Byte 5: Identifier(low byte)
        packet.push((self.sequence_number >> 8) as u8);     //Byte 6: Sequence number(high byte)
        packet.push((self.sequence_number & 0xFF) as u8);   //Byte 7: Sequence number(low byte)

        // Payload
        packet.extend_from_slice(&payload_clone);

        // Calculate checksum over entire packet
        let cs = checksum(&packet);
        packet[2] = (cs >> 8) as u8;
        packet[3] = (cs & 0xFF) as u8;
        
        packet
    }
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



fn main() -> io::Result<()>{
    let dest_ip = "127.0.0.1".parse::<IpAddr>().unwrap();

    //Use UDPSocket to access the raw file descriptor
    let socket = UdpSocket::bind("0.0.0.0:0")?; 
    socket.connect((dest_ip, 0))?;
    let raw_fd = socket.as_raw_fd();

    let identifier = 0x1234;
    let sequence = 1;
    let payload = b"Hello PING-RS".to_vec();

    let icmp_packet = IcmpEcho::new(identifier, sequence, payload);
    let packet = icmp_packet.to_bytes();

    let start = Instant::now();
    let bytes_sent = unsafe {
        libc::sendto(
            raw_fd,
            packet.as_ptr() as *const _,
            packet.len(),
            0,
            std::ptr::null(),
            0,
        )
    };

    if bytes_sent < 0 {
        return Err(io::Error::last_os_error());
    }

    let mut buf = [0u8; 1024];
    // let len = socket.recv(&mut buf);
    let len = unsafe {
    libc::recv(
        raw_fd,
        buf.as_mut_ptr() as *mut _,
        buf.len(),
        0,
    )
};

if len < 0 {
    return Err(io::Error::last_os_error());
}

    let elapsed = start.elapsed();

    println!(
        "Reply from {}: bytes={:?} time={:.2?}",
        dest_ip,
        len,
        elapsed
    );

    Ok(())

}
