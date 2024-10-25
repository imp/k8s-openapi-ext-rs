use super::*;

pub trait EphemeralContainerGetExt {
    fn myself(&self) -> &corev1::EphemeralContainer;
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

impl EphemeralContainerGetExt for corev1::EphemeralContainer {
    #[inline(always)]
    fn myself(&self) -> &corev1::EphemeralContainer {
        self
    }
}
