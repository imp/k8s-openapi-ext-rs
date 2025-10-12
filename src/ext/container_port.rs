use super::*;

pub trait ContainerPortExt: Sized {
    fn new(port: impl Into<i32>, protocol: impl ToString) -> Self;

    fn tcp(port: impl Into<i32>) -> Self {
        Self::new(port, "TCP")
    }

    fn udp(port: impl Into<i32>) -> Self {
        Self::new(port, "UDP")
    }

    fn sctp(port: impl Into<i32>) -> Self {
        Self::new(port, "SCTP")
    }

    fn name(self, name: impl ToString) -> Self;

    fn host_ip(self, ip: impl ToString) -> Self;

    fn host_port(self, port: impl Into<i32>) -> Self;
}

impl ContainerPortExt for corev1::ContainerPort {
    fn new(port: impl Into<i32>, protocol: impl ToString) -> Self {
        let protocol = Some(protocol.to_string());
        Self {
            container_port: port.into(),
            protocol,
            // host_ip: (),
            // host_port: (),
            // name: (),
            ..default()
        }
    }

    fn name(self, name: impl ToString) -> Self {
        let name = Some(name.to_string());
        Self { name, ..self }
    }

    fn host_ip(self, ip: impl ToString) -> Self {
        let host_ip = Some(ip.to_string());
        Self { host_ip, ..self }
    }

    fn host_port(self, port: impl Into<i32>) -> Self {
        let host_port = Some(port.into());
        Self { host_port, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_ip() {
        let ip: std::net::IpAddr = "127.0.0.1".parse().unwrap();
        let port = corev1::ContainerPort::tcp(1234).host_ip(ip);
        assert_eq!(port.host_ip.as_deref(), Some("127.0.0.1"));
    }
}
