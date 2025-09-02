use super::*;

pub use ephemeral::EphemeralContainerGetExt;
pub use state::ContainerStateTerminatedGetExt;
pub use state::ContainerStateWaitingGetExt;
pub use status::ContainerStatusGetExt;

mod ephemeral;
mod state;
mod status;

pub trait ContainerGetExt {
    fn container(&self) -> &corev1::Container;

    fn name(&self) -> &str {
        self.container().name.as_str()
    }

    fn args(&self) -> Option<&[String]> {
        self.container().args.as_deref()
    }

    fn image(&self) -> Option<&str> {
        self.container().image.as_deref()
    }

    fn image_pull_policy(&self) -> Option<&str> {
        self.container().image_pull_policy.as_deref()
    }

    fn ports(&self) -> Option<&[corev1::ContainerPort]> {
        self.container().ports.as_deref()
    }

    fn liveness_probe(&self) -> Option<&corev1::Probe> {
        self.container().liveness_probe.as_ref()
    }

    fn readiness_probe(&self) -> Option<&corev1::Probe> {
        self.container().readiness_probe.as_ref()
    }

    fn startup_probe(&self) -> Option<&corev1::Probe> {
        self.container().startup_probe.as_ref()
    }

    fn resources(&self) -> Option<&corev1::ResourceRequirements> {
        self.container().resources.as_ref()
    }

    fn restart_policy(&self) -> Option<&str> {
        self.container().restart_policy.as_deref()
    }

    fn security_context(&self) -> Option<&corev1::SecurityContext> {
        self.container().security_context.as_ref()
    }

    fn working_dir(&self) -> Option<&str> {
        self.container().working_dir.as_deref()
    }

    fn port_by_name(&self, name: impl AsRef<str>) -> Option<&corev1::ContainerPort> {
        let name = name.as_ref();
        self.ports()?
            .iter()
            .find(|port| port.name.as_deref() == Some(name))
    }

    fn is_restartable(&self) -> bool {
        self.restart_policy() == Some("Always")
    }
}

impl ContainerGetExt for corev1::Container {
    #[inline(always)]
    fn container(&self) -> &corev1::Container {
        self
    }
}
