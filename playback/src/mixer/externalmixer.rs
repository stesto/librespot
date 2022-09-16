use std::{net::{SocketAddr, IpAddr, Ipv4Addr, TcpStream}, time::Duration, io::Write, io::Read};

use super::{Mixer, MixerConfig};

#[derive(Clone)]
pub struct ExternalMixer {
    ip: SocketAddr,
    timeout: Duration,

    request_volume_cmd: [u8; 2],
    set_volume_cmd: [u8; 3],
}

impl Mixer for ExternalMixer {

    fn open(_: MixerConfig) -> Self {
        Self {
            ip: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 55550),
            timeout: Duration::from_millis(100),

            request_volume_cmd: [
                0, // librespot id
                0  // get volume
            ],
            set_volume_cmd: [
                0, // librespot id
                1, // set volume
                0  // placeholder for volume value
            ]
        }
    }

    fn volume(&self) -> u16 {
        let mut _volume: u16 = 0;

        if let Ok(mut stream) = TcpStream::connect_timeout(&self.ip, self.timeout) {
            if let Ok(_) = stream.write(&self.request_volume_cmd) {
                let mut buf: [u8; 1] = [0];
                if let Ok(_) = stream.read_exact(&mut buf) {
                    _volume = u16::MAX / (u8::MAX as u16) * (buf[0] as u16);
                }
            }
        }

        _volume
    }

    fn set_volume(&self, volume: u16) {
        if let Ok(mut stream) = TcpStream::connect_timeout(&self.ip, self.timeout) {
            let mut _volume_cmd = &mut self.set_volume_cmd.clone();
            _volume_cmd[2] = (volume / (u16::MAX / u8::MAX as u16)) as u8;
            let _ = stream.write(_volume_cmd);
        }
    }
}

impl ExternalMixer {
    pub const NAME: &'static str = "external";
}