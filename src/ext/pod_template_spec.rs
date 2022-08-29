use super::*;

pub trait PodTemplateSpecExt {
    fn new(name: impl ToString) -> Self;

    fn labels(self, labels: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self;

    fn pod_spec(self, spec: corev1::PodSpec) -> Self;
}

impl PodTemplateSpecExt for corev1::PodTemplateSpec {
    fn new(name: impl ToString) -> Self {
        Self {
            metadata: Some(metadata(name)),
            spec: None,
        }
    }

    fn labels(self, labels: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self {
        let labels = labels
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        let mut metadata = self.metadata.unwrap_or_default();
        metadata.labels = Some(labels);
        Self {
            metadata: Some(metadata),
            ..self
        }
    }

    fn pod_spec(self, spec: corev1::PodSpec) -> Self {
        Self {
            spec: Some(spec),
            ..self
        }
    }
}
