use super::*;

pub trait StatefulSetGetExt {
    fn spec(&self) -> Option<&appsv1::StatefulSetSpec>;

    fn status(&self) -> Option<&appsv1::StatefulSetStatus>;

    // From spec
    fn min_ready_seconds(&self) -> Option<i32> {
        self.spec()?.min_ready_seconds
    }

    // From status
    fn available_replicas(&self) -> Option<i32> {
        self.status()?.available_replicas
    }
    fn collision_count(&self) -> Option<i32> {
        self.status()?.collision_count
    }
    fn conditions(&self) -> Option<&[appsv1::StatefulSetCondition]> {
        self.status()?.conditions.as_deref()
    }
    fn current_replicas(&self) -> Option<i32> {
        self.status()?.current_replicas
    }
    fn current_revision(&self) -> Option<&str> {
        self.status()?.current_revision.as_deref()
    }
    fn observed_generation(&self) -> Option<i64> {
        self.status()?.observed_generation
    }
    fn ready_replicas(&self) -> Option<i32> {
        self.status()?.ready_replicas
    }
    fn status_replicas(&self) -> i32 {
        self.status()
            .map(|status| status.replicas)
            .unwrap_or_default()
    }
    fn update_revision(&self) -> Option<&str> {
        self.status()?.update_revision.as_deref()
    }
    fn updated_replicas(&self) -> Option<i32> {
        self.status()?.updated_replicas
    }
}

impl StatefulSetGetExt for appsv1::StatefulSet {
    fn spec(&self) -> Option<&appsv1::StatefulSetSpec> {
        self.spec.as_ref()
    }

    fn status(&self) -> Option<&appsv1::StatefulSetStatus> {
        self.status.as_ref()
    }
}
