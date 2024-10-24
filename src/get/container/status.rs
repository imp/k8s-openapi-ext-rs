use super::*;

pub trait ContainerStatusGetExt {
    fn myself(&self) -> &corev1::ContainerStatus;

    fn name(&self) -> &str {
        &self.myself().name
    }

    fn allocated_resources(&self) -> Option<&BTreeMap<String, resource::Quantity>> {
        self.myself().allocated_resources.as_ref()
    }

    fn allocated_resources_status(&self) -> Option<&[corev1::ResourceStatus]> {
        self.myself().allocated_resources_status.as_deref()
    }

    fn container_id(&self) -> Option<&str> {
        self.myself().container_id.as_deref()
    }
    fn image(&self) -> &str {
        &self.myself().image
    }

    fn image_id(&self) -> &str {
        &self.myself().image_id
    }

    fn last_state(&self) -> Option<&corev1::ContainerState> {
        self.myself().last_state.as_ref()
    }

    fn ready(&self) -> bool {
        self.myself().ready
    }

    fn resources(&self) -> Option<&corev1::ResourceRequirements> {
        self.myself().resources.as_ref()
    }

    fn restart_count(&self) -> i32 {
        self.myself().restart_count
    }

    fn started(&self) -> Option<bool> {
        self.myself().started
    }

    fn state(&self) -> Option<&corev1::ContainerState> {
        self.myself().state.as_ref()
    }

    fn user(&self) -> Option<&corev1::ContainerUser> {
        self.myself().user.as_ref()
    }

    fn volume_mounts(&self) -> Option<&[corev1::VolumeMountStatus]> {
        self.myself().volume_mounts.as_deref()
    }
}

impl ContainerStatusGetExt for corev1::ContainerStatus {
    #[inline(always)]
    fn myself(&self) -> &corev1::ContainerStatus {
        self
    }
}
