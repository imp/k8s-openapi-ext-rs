use super::*;

pub trait ConditionGetExt {
    fn myself(&self) -> &metav1::Condition;

    fn r#type_(&self) -> &str {
        &self.myself().type_
    }

    fn status(&self) -> &str {
        &self.myself().status
    }

    fn last_transition_time(&self) -> &metav1::Time {
        &self.myself().last_transition_time
    }

    fn reason(&self) -> &str {
        &self.myself().reason
    }

    fn message(&self) -> &str {
        &self.myself().message
    }

    fn observed_generation(&self) -> Option<i64> {
        self.myself().observed_generation
    }
}

impl ConditionGetExt for metav1::Condition {
    #[inline(always)]
    fn myself(&self) -> &metav1::Condition {
        self
    }
}
