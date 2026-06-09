use std::net::IpAddr;

pub type DeviceId = String;

#[derive(Debug, Clone)]
pub struct Device {
    id: DeviceId,
    name: String,
    ip: IpAddr,
    port: u16,
}
