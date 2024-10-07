use std::path::Path;

use super::*;

/// Builders for `corev1::Probe` objects
pub trait ProbeExt {
    /// HTTP get probe
    fn http_get(path: impl AsRef<Path>, port: u16) -> Self;

    /// TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
    fn tcp_socket(port: u16) -> Self;

    /// gRPC probe
    ///
    fn grpc<'a>(port: i32, service: impl Into<Option<&'a str>>) -> Self;

    /// Minimum consecutive failures for the probe to be considered failed after having succeeded.
    /// Defaults to 3. Minimum value is 1.
    ///
    fn failure_threshold(self, threshold: i32) -> Self;

    /// Minimum consecutive successes for the probe to be considered successful after having failed.
    /// Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    ///
    fn success_threshold(self, threshold: i32) -> Self;

    fn initial_delay_seconds(self, seconds: i32) -> Self;

    fn period_seconds(self, seconds: i32) -> Self;

    fn timeout_seconds(self, seconds: i32) -> Self;

    fn termination_grace_period_seconds(self, seconds: i64) -> Self;
}

impl ProbeExt for corev1::Probe {
    fn http_get(path: impl AsRef<Path>, port: u16) -> Self {
        let path = Some(path.as_ref().display().to_string());
        let port = intstr::IntOrString::Int(port.into());
        let http_get = Some(corev1::HTTPGetAction {
            path,
            port,
            // host: todo!(),
            // http_headers: todo!(),
            // scheme: todo!(),
            ..default()
        });
        Self {
            http_get,
            // exec: todo!(),
            // failure_threshold: todo!(),
            // initial_delay_seconds: todo!(),
            // period_seconds: todo!(),
            // success_threshold: todo!(),
            // tcp_socket: todo!(),
            // termination_grace_period_seconds: todo!(),
            // timeout_seconds: todo!(),
            ..default()
        }
    }

    fn tcp_socket(port: u16) -> Self {
        let tcp_socket = Some(corev1::TCPSocketAction {
            port: intstr::IntOrString::Int(port.into()),
            // host: todo!(),
            ..default()
        });
        Self {
            tcp_socket,
            // exec: todo!(),
            // failure_threshold: todo!(),
            // grpc: todo!(),
            // http_get: todo!(),
            // initial_delay_seconds: todo!(),
            // period_seconds: todo!(),
            // success_threshold: todo!(),
            // termination_grace_period_seconds: todo!(),
            // timeout_seconds: todo!(),
            ..default()
        }
    }

    fn grpc<'a>(port: i32, service: impl Into<Option<&'a str>>) -> Self {
        let service = service.into().map(|service| service.to_string());
        let grpc = Some(corev1::GRPCAction { port, service });
        Self {
            grpc,
            // exec: todo!(),
            // failure_threshold: todo!(),
            // http_get: todo!(),
            // initial_delay_seconds: todo!(),
            // period_seconds: todo!(),
            // success_threshold: todo!(),
            // tcp_socket: todo!(),
            // termination_grace_period_seconds: todo!(),
            // timeout_seconds: todo!(),
            ..default()
        }
    }

    fn failure_threshold(mut self, threshold: i32) -> Self {
        self.failure_threshold = Some(threshold);
        self
    }

    fn success_threshold(mut self, threshold: i32) -> Self {
        self.success_threshold = Some(threshold);
        self
    }

    fn initial_delay_seconds(mut self, seconds: i32) -> Self {
        self.initial_delay_seconds = Some(seconds);
        self
    }

    fn period_seconds(mut self, seconds: i32) -> Self {
        self.period_seconds = Some(seconds);
        self
    }

    fn timeout_seconds(mut self, seconds: i32) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    fn termination_grace_period_seconds(mut self, seconds: i64) -> Self {
        self.termination_grace_period_seconds = Some(seconds);
        self
    }
}
