use super::*;

pub use condition::PodConditionGetExt;

mod condition;

pub trait PodGetExt {
    fn spec(&self) -> Option<&corev1::PodSpec>;

    fn status(&self) -> Option<&corev1::PodStatus>;

    // From spec
    fn containers(&self) -> Option<&[corev1::Container]> {
        self.spec().map(|spec| spec.containers.as_ref())
    }

    fn ephemeral_containers(&self) -> Option<&[corev1::EphemeralContainer]> {
        self.spec()?.ephemeral_containers.as_deref()
    }

    fn init_containers(&self) -> Option<&[corev1::Container]> {
        self.spec()?.init_containers.as_deref()
    }

    fn node_selector(&self) -> Option<&BTreeMap<String, String>> {
        self.spec()?.node_selector.as_ref()
    }

    fn resource_claims(&self) -> Option<&[corev1::PodResourceClaim]> {
        self.spec()?.resource_claims.as_deref()
    }

    fn tolerations(&self) -> Option<&[corev1::Toleration]> {
        self.spec()?.tolerations.as_deref()
    }

    // From status
    fn message(&self) -> Option<&str> {
        self.status()?.message.as_deref()
    }

    fn phase(&self) -> Option<&str> {
        self.status()?.phase.as_deref()
    }

    fn qos_class(&self) -> Option<&str> {
        self.status()?.qos_class.as_deref()
    }

    fn reason(&self) -> Option<&str> {
        self.status()?.reason.as_deref()
    }

    fn resize(&self) -> Option<&str> {
        self.status()?.resize.as_deref()
    }

    fn conditions(&self) -> Option<&[corev1::PodCondition]> {
        self.status()?.conditions.as_deref()
    }

    fn container_statuses(&self) -> Option<&[corev1::ContainerStatus]> {
        self.status()?.container_statuses.as_deref()
    }

    fn ephemeral_container_statuses(&self) -> Option<&[corev1::ContainerStatus]> {
        self.status()?.ephemeral_container_statuses.as_deref()
    }

    fn init_container_statuses(&self) -> Option<&[corev1::ContainerStatus]> {
        self.status()?.init_container_statuses.as_deref()
    }

    fn nominated_node_name(&self) -> Option<&str> {
        self.status()?.nominated_node_name.as_deref()
    }

    fn resource_claim_statuses(&self) -> Option<&[corev1::PodResourceClaimStatus]> {
        self.status()?.resource_claim_statuses.as_deref()
    }

    fn host_ip(&self) -> Option<&str> {
        self.status()?.host_ip.as_deref()
    }

    fn host_ips(&self) -> Option<&[corev1::HostIP]> {
        self.status()?.host_ips.as_deref()
    }

    fn pod_ip(&self) -> Option<&str> {
        self.status()?.pod_ip.as_deref()
    }

    fn pod_ips(&self) -> Option<&[corev1::PodIP]> {
        self.status()?.pod_ips.as_deref()
    }

    fn start_time(&self) -> Option<&metav1::Time> {
        self.status()?.start_time.as_ref()
    }

    fn condition(&self, type_: &str) -> Option<&corev1::PodCondition> {
        self.conditions()?
            .iter()
            .find(|condition| condition.type_ == type_)
    }

    fn is_ready(&self) -> bool {
        self.condition("Ready")
            .is_some_and(|condition| condition.is_true())
    }
}

impl PodGetExt for corev1::Pod {
    fn spec(&self) -> Option<&corev1::PodSpec> {
        self.spec.as_ref()
    }

    fn status(&self) -> Option<&corev1::PodStatus> {
        self.status.as_ref()
    }
}

impl PodGetExt for corev1::PodStatus {
    fn spec(&self) -> Option<&corev1::PodSpec> {
        None
    }

    fn status(&self) -> Option<&corev1::PodStatus> {
        Some(self)
    }
}

impl PodGetExt for corev1::PodTemplate {
    fn spec(&self) -> Option<&corev1::PodSpec> {
        self.template.as_ref()?.spec.as_ref()
    }

    fn status(&self) -> Option<&corev1::PodStatus> {
        None
    }
}

impl PodGetExt for corev1::PodTemplateSpec {
    fn spec(&self) -> Option<&corev1::PodSpec> {
        self.spec.as_ref()
    }

    fn status(&self) -> Option<&corev1::PodStatus> {
        None
    }
}
