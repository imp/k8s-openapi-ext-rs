use super::*;

pub trait ServicePortExt {
    /// Construct new `ServicePort`
    ///
    fn new(name: impl ToString, port: i32) -> Self;

    /// Set targetPort
    ///
    fn target_port(self, port: impl ToIntOrString) -> Self;

    /// Set protocol
    ///
    fn protocol(self, protocol: impl ToString) -> Self;

    /// Create UDP ServicePort
    ///
    fn udp(name: impl ToString, port: i32) -> Self
    where
        Self: std::marker::Sized,
    {
        Self::new(name, port).protocol("UDP")
    }

    /// Create SCTP ServicePort
    ///
    fn sctp(name: impl ToString, port: i32) -> Self
    where
        Self: std::marker::Sized,
    {
        Self::new(name, port).protocol("SCTP")
    }
}

impl ServicePortExt for corev1::ServicePort {
    fn new(name: impl ToString, port: i32) -> Self {
        let name = Some(name.to_string());
        Self {
            name,
            port,
            // app_protocol: todo!(),
            // node_port: todo!(),
            // protocol: todo!(),
            // target_port: todo!(),
            ..default()
        }
    }

    fn target_port(self, port: impl ToIntOrString) -> Self {
        let target_port = Some(port.to_int_or_string());
        Self {
            target_port,
            ..self
        }
    }

    fn protocol(self, protocol: impl ToString) -> Self {
        let protocol = Some(protocol.to_string());
        Self { protocol, ..self }
    }
}
