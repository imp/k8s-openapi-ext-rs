use super::*;

pub trait ReplicaSetConditionGetExt {
    fn myself(&self) -> &appsv1::ReplicaSetCondition;

    fn last_transition_time(&self) -> Option<&metav1::Time> {
        self.myself().last_transition_time.as_ref()
    }

    fn message(&self) -> Option<&str> {
        self.myself().message.as_deref()
    }

    fn reason(&self) -> Option<&str> {
        self.myself().reason.as_deref()
    }

    fn status(&self) -> &str {
        &self.myself().status
    }

    fn r#type(&self) -> &str {
        &self.myself().type_
    }
}

impl ReplicaSetConditionGetExt for appsv1::ReplicaSetCondition {
    #[inline(always)]
    fn myself(&self) -> &appsv1::ReplicaSetCondition {
        self
    }
}
