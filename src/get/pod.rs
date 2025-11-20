use super::*;

pub use condition::PodConditionGetExt;

mod condition;

pub trait PodGetExt {
    #[deprecated(note = "Use corev1::PodCondition::POD_SCHEDULED instead")]
    const POD_SCHEDULED: &str = "PodScheduled";

    /// The Pod has been accepted by the Kubernetes cluster, but one or more
    /// of the containers has not been set up and made ready to run.
    /// This includes time a Pod spends waiting to be scheduled as well as the
    /// time spent downloading container images over the network.
    const POD_PENDING: &str = "Pending";

    /// The Pod has been bound to a node, and all of the containers have been
    /// created. At least one container is still running, or is in the process
    /// of starting or restarting.
    const POD_RUNNING: &str = "Running";

    /// All containers in the Pod have terminated in success, and will not
    /// be restarted.
    const POD_SUCCEEDED: &str = "Succeeded";

    /// All containers in the Pod have terminated, and at least one container
    /// has terminated in failure. That is, the container either exited with
    /// non-zero status or was terminated by the system, and is not set for
    /// automatic restarting.
    const POD_FAILED: &str = "Failed";

    /// For some reason the state of the Pod could not be obtained.
    /// This phase typically occurs due to an error in communicating with
    /// the node where the Pod should be running.
    const POD_UNKNOWN: &str = "Unknown";

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

    fn readiness_gates(&self) -> Option<&[corev1::PodReadinessGate]> {
        self.spec()?.readiness_gates.as_deref()
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

    fn is_running(&self) -> bool {
        self.phase().is_some_and(|phase| phase == Self::POD_RUNNING)
    }

    fn is_ready(&self) -> bool {
        let gates = self
            .readiness_gates()
            .unwrap_or_default()
            .iter()
            .all(|gate| {
                self.condition(&gate.condition_type)
                    .is_some_and(|condition| condition.is_true())
            });

        let ready = self
            .condition(corev1::PodCondition::POD_READY)
            .is_some_and(|condition| condition.is_true());

        gates && ready
    }

    fn pod_scheduled_reason(&self) -> Option<&str> {
        self.condition(corev1::PodCondition::POD_SCHEDULED)?
            .reason()
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
