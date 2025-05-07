use super::*;

pub use condition::ReplicaSetConditionGetExt;

mod condition;

pub trait ReplicaSetGetExt {
    fn spec(&self) -> Option<&appsv1::ReplicaSetSpec>;
    fn status(&self) -> Option<&appsv1::ReplicaSetStatus>;

    fn min_ready_seconds(&self) -> Option<i32> {
        self.spec()?.min_ready_seconds
    }

    fn spec_replicas(&self) -> Option<i32> {
        self.spec()?.replicas
    }

    fn selector(&self) -> Option<&metav1::LabelSelector> {
        self.spec().map(|spec| &spec.selector)
    }

    fn template(&self) -> Option<&corev1::PodTemplateSpec> {
        self.spec()?.template.as_ref()
    }

    fn available_replicas(&self) -> Option<i32> {
        self.status()?.available_replicas
    }

    fn ready_replicas(&self) -> Option<i32> {
        self.status()?.ready_replicas
    }

    fn fully_labeled_replicas(&self) -> Option<i32> {
        self.status()?.fully_labeled_replicas
    }
    fn status_replicas(&self) -> Option<i32> {
        self.status().map(|status| status.replicas)
    }

    fn observed_generation(&self) -> Option<i64> {
        self.status()?.observed_generation
    }

    fn conditions(&self) -> Option<&[appsv1::ReplicaSetCondition]> {
        self.status()?.conditions.as_deref()
    }
}

impl ReplicaSetGetExt for appsv1::ReplicaSet {
    fn spec(&self) -> Option<&appsv1::ReplicaSetSpec> {
        self.spec.as_ref()
    }

    fn status(&self) -> Option<&appsv1::ReplicaSetStatus> {
        self.status.as_ref()
    }
}
