use super::*;

pub trait ContainerStatusGetExt {
    fn container_status(&self) -> &corev1::ContainerStatus;

    fn name(&self) -> &str {
        &self.container_status().name
    }

    fn allocated_resources(&self) -> Option<&BTreeMap<String, resource::Quantity>> {
        self.container_status().allocated_resources.as_ref()
    }

    fn container_id(&self) -> Option<&str> {
        self.container_status().container_id.as_deref()
    }
    fn image(&self) -> &str {
        &self.container_status().image
    }

    fn image_id(&self) -> &str {
        &self.container_status().image_id
    }

    fn last_state(&self) -> Option<&corev1::ContainerState> {
        self.container_status().last_state.as_ref()
    }

    fn ready(&self) -> bool {
        self.container_status().ready
    }

    fn resources(&self) -> Option<&corev1::ResourceRequirements> {
        self.container_status().resources.as_ref()
    }

    fn restart_count(&self) -> i32 {
        self.container_status().restart_count
    }

    fn started(&self) -> Option<bool> {
        self.container_status().started
    }

    fn state(&self) -> Option<&corev1::ContainerState> {
        self.container_status().state.as_ref()
    }

    openapi::k8s_if_ge_1_31! {
        fn allocated_resources_status(&self) -> Option<&[corev1::ResourceStatus]> {
            self.container_status().allocated_resources_status.as_deref()
        }
        fn user(&self) -> Option<&corev1::ContainerUser> {
            self.container_status().user.as_ref()
        }

        fn volume_mounts(&self) -> Option<&[corev1::VolumeMountStatus]> {
            self.container_status().volume_mounts.as_deref()
        }
    }
}

impl ContainerStatusGetExt for corev1::ContainerStatus {
    #[inline(always)]
    fn container_status(&self) -> &corev1::ContainerStatus {
        self
    }
}
