use super::*;

pub trait NamespaceGetExt {
    fn spec(&self) -> Option<&corev1::NamespaceSpec>;

    fn status(&self) -> Option<&corev1::NamespaceStatus>;

    fn finalizers(&self) -> Option<&[String]> {
        self.spec()?.finalizers.as_deref()
    }

    fn conditions(&self) -> Option<&[corev1::NamespaceCondition]> {
        self.status()?.conditions.as_deref()
    }

    fn phase(&self) -> Option<&str> {
        self.status()?.phase.as_deref()
    }

    fn condition(&self, r#type: impl AsRef<str>) -> Option<&corev1::NamespaceCondition> {
        let r#type = r#type.as_ref();
        self.conditions()?
            .iter()
            .find(|condition| condition.type_ == r#type)
    }
}

impl NamespaceGetExt for corev1::Namespace {
    fn spec(&self) -> Option<&corev1::NamespaceSpec> {
        self.spec.as_ref()
    }

    fn status(&self) -> Option<&corev1::NamespaceStatus> {
        self.status.as_ref()
    }
}
