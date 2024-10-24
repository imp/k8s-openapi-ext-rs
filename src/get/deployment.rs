use super::*;

pub trait DeploymentGetExt {
    fn spec(&self) -> Option<&appsv1::DeploymentSpec>;

    fn status(&self) -> Option<&appsv1::DeploymentStatus>;

    // From spec

    fn min_ready_seconds(&self) -> Option<i32> {
        self.spec()?.min_ready_seconds
    }

    fn paused(&self) -> Option<bool> {
        self.spec()?.paused
    }

    fn progress_deadline_seconds(&self) -> Option<i32> {
        self.spec()?.progress_deadline_seconds
    }

    fn revision_history_limit(&self) -> Option<i32> {
        self.spec()?.revision_history_limit
    }

    fn selector(&self) -> Option<&metav1::LabelSelector> {
        self.spec().map(|spec| &spec.selector)
    }

    fn strategy(&self) -> Option<&appsv1::DeploymentStrategy> {
        self.spec()?.strategy.as_ref()
    }

    fn template(&self) -> Option<&corev1::PodTemplateSpec> {
        self.spec().map(|spec| &spec.template)
    }

    fn spec_replicas(&self) -> Option<i32> {
        self.spec()?.replicas
    }

    // From status
    fn conditions(&self) -> Option<&[appsv1::DeploymentCondition]> {
        self.status()?.conditions.as_deref()
    }

    fn status_replicas(&self) -> Option<i32> {
        self.status()?.replicas
    }

    fn available_replicas(&self) -> Option<i32> {
        self.status()?.available_replicas
    }

    fn ready_replicas(&self) -> Option<i32> {
        self.status()?.ready_replicas
    }

    fn unavailable_replicas(&self) -> Option<i32> {
        self.status()?.unavailable_replicas
    }

    fn updated_replicas(&self) -> Option<i32> {
        self.status()?.updated_replicas
    }

    fn collision_count(&self) -> Option<i32> {
        self.status()?.collision_count
    }

    fn observed_generation(&self) -> Option<i64> {
        self.status()?.observed_generation
    }
}

impl DeploymentGetExt for appsv1::Deployment {
    fn spec(&self) -> Option<&appsv1::DeploymentSpec> {
        self.spec.as_ref()
    }

    fn status(&self) -> Option<&appsv1::DeploymentStatus> {
        self.status.as_ref()
    }
}

impl DeploymentGetExt for appsv1::DeploymentSpec {
    fn spec(&self) -> Option<&appsv1::DeploymentSpec> {
        Some(self)
    }

    fn status(&self) -> Option<&appsv1::DeploymentStatus> {
        None
    }
}

impl DeploymentGetExt for appsv1::DeploymentStatus {
    fn spec(&self) -> Option<&appsv1::DeploymentSpec> {
        None
    }

    fn status(&self) -> Option<&appsv1::DeploymentStatus> {
        Some(self)
    }
}
