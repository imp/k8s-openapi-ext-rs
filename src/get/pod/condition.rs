use super::*;

pub trait PodConditionGetExt {
    fn myself(&self) -> &corev1::PodCondition;

    fn status(&self) -> &str {
        &self.myself().status
    }

    fn r#type(&self) -> &str {
        &self.myself().type_
    }

    fn message(&self) -> Option<&str> {
        self.myself().message.as_deref()
    }

    fn reason(&self) -> Option<&str> {
        self.myself().reason.as_deref()
    }

    fn last_probe_time(&self) -> Option<&metav1::Time> {
        self.myself().last_probe_time.as_ref()
    }

    fn last_transition_time(&self) -> Option<&metav1::Time> {
        self.myself().last_transition_time.as_ref()
    }

    fn is_true(&self) -> bool {
        self.status() == "True"
    }

    fn is_false(&self) -> bool {
        self.status() == "False"
    }

    fn is_unknown(&self) -> bool {
        self.status() == "Unknown"
    }
}

impl PodConditionGetExt for corev1::PodCondition {
    #[inline(always)]
    fn myself(&self) -> &corev1::PodCondition {
        self
    }
}
