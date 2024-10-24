use super::*;

pub use status::ContainerStatusGetExt;

mod status;

pub trait ContainerGetExt {
    fn myself(&self) -> &corev1::Container;

    fn name(&self) -> &str {
        &self.myself().name
    }

    fn ports(&self) -> Option<&[corev1::ContainerPort]> {
        self.myself().ports.as_deref()
    }

    fn liveness_probe(&self) -> Option<&corev1::Probe> {
        self.myself().liveness_probe.as_ref()
    }

    fn readiness_probe(&self) -> Option<&corev1::Probe> {
        self.myself().readiness_probe.as_ref()
    }

    fn startup_probe(&self) -> Option<&corev1::Probe> {
        self.myself().startup_probe.as_ref()
    }

    fn resources(&self) -> Option<&corev1::ResourceRequirements> {
        self.myself().resources.as_ref()
    }

    fn port_by_name(&self, name: &str) -> Option<&corev1::ContainerPort> {
        self.ports()?
            .iter()
            .find(|port| port.name.as_deref() == Some(name))
    }
}

impl ContainerGetExt for corev1::Container {
    #[inline(always)]
    fn myself(&self) -> &corev1::Container {
        self
    }
}
