use std::error::Error;
use std::fmt;
use std::net::UdpSocket;
use std::time::Duration;

pub type MagicPacketResult<T> = Result<T, MagicPacketError>;

#[derive(Debug)]
pub enum MagicPacketError {
    FailedToBindSocket(String),
    FailedToSetSocketReadTimeOut(String),
    FailedToSetSocketForBroadcast(String),
    FailedToSendPacket(String),
}

impl fmt::Display for MagicPacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for MagicPacketError {}

/// Will find the broadcast IP address and append the provided
/// port number. If port provided is None, will use the default
/// value '9'.
pub fn brodcast_address(port: Option<u16>) -> Option<String> {
    let port = port.unwrap_or(9);
    let Ok(addrs) = nix::ifaddrs::getifaddrs() else {
        return None;
    };
    for addr in addrs {
        if let Some(broadcast_addr) = addr.broadcast {
            let bc_addr = broadcast_addr.to_string();
            let port_index = bc_addr.find(':').unwrap_or(bc_addr.len());
            let no_port = &bc_addr[0..port_index];
            let final_addr = format!("{}:{}", no_port, port);
            return Some(final_addr);
        }
    }
    None
}

/// Creates a Magic Packet.
/// The format of the packet is 48 bits set to '1' followed by
/// [mac] 16 times consecutively.
/// ex. FFFFFFFFFFFFmacmacmacmacmacmacmacmacmacmacmacmacmacmacmacmac
/// The packet does not include MAC address seprators (ie ':' or '-')
/// however this function expects the use of ':' as separator for the
/// argument supplied for the [mac] parameter.
pub fn create_magic_packet(mac: &str) -> Vec<u8> {
    let mac_bytes = mac
        .split(':')
        .flat_map(|x| u8::from_str_radix(x, 16).ok())
        .collect::<Vec<u8>>();

    let mut packet = vec![0xff; 6];
    packet.extend(vec![mac_bytes.as_slice(); 16].concat());
    packet
}

/// Sends the Magic Packet to the specified broadcast_addr.
pub fn send_magic_packet(
    packet: &[u8],
    broadcast_addr: &str,
    read_timeout: u64,
) -> MagicPacketResult<usize> {
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0")
        .map_err(|err| MagicPacketError::FailedToBindSocket(err.to_string()))?;
    socket
        .set_read_timeout(Some(Duration::new(read_timeout, 0)))
        .map_err(|err| MagicPacketError::FailedToSetSocketReadTimeOut(err.to_string()))?;
    socket
        .set_broadcast(true)
        .map_err(|err| MagicPacketError::FailedToSetSocketForBroadcast(err.to_string()))?;

    let num_bytes = socket
        .send_to(packet, broadcast_addr)
        .map_err(|err| MagicPacketError::FailedToSendPacket(err.to_string()))?;
    Ok(num_bytes)
}

#[test]
fn test_create_magic_packet() {
    const MAC: &str = "12:44:56:C8:12:A8";
    let packet = create_magic_packet(MAC);
    let str: String = packet.iter().map(|f| format!("{:02X?}", f)).collect();
    assert_eq!(str, "FFFFFFFFFFFF124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8");
}
