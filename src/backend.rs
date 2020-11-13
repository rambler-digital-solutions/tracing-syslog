use std::io;
use std::io::ErrorKind;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

#[derive(Debug)]
pub enum LoggerBackendBuilder {
    Upd(UdpSocket, SocketAddr),
}

impl LoggerBackendBuilder {
    pub fn udp<T: ToSocketAddrs>(address: T) -> io::Result<LoggerBackendBuilder> {
        let address = address
            .to_socket_addrs()?
            .next()
            .ok_or(ErrorKind::InvalidData)?;
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(LoggerBackendBuilder::Upd(socket, address))
    }
    pub fn finish(self) -> LoggerBackend {
        match self {
            LoggerBackendBuilder::Upd(socket, address) => LoggerBackend::Upd(socket, address),
        }
    }
}

#[derive(Debug)]
pub enum LoggerBackend {
    Upd(UdpSocket, SocketAddr),
}

impl LoggerBackend {
    pub fn send(&self, message: &[u8]) -> io::Result<usize> {
        match self {
            LoggerBackend::Upd(socket, address) => socket.send_to(message, address),
        }
    }
}

impl Default for LoggerBackend {
    fn default() -> Self {
        LoggerBackendBuilder::udp("127.0.0.1:514").unwrap().finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_backend_builder_udp() {
        let builder = LoggerBackendBuilder::udp("127.0.0.1:8080");
        assert!(&builder.is_ok());
    }

    #[test]
    fn test_logger_backend_builder_finish() {
        let backend = LoggerBackendBuilder::udp("127.0.0.1:8080")
            .unwrap()
            .finish();
        match backend {
            LoggerBackend::Upd(_socket, address) => {
                assert_eq!(address.to_string(), "127.0.0.1:8080")
            }
        }
    }
}
