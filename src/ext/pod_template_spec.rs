use super::*;

pub trait PodTemplateSpecExt {
    fn new(name: impl ToString) -> Self;

    fn labels(self, labels: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self;

    fn pod_spec(self, spec: corev1::PodSpec) -> Self;
}

impl PodTemplateSpecExt for corev1::PodTemplateSpec {
    fn new(name: impl ToString) -> Self {
        let name = Some(name.to_string());
        let metadata = metav1::ObjectMeta {
            name,
            // annotations: todo!(),
            // cluster_name: todo!(),
            // creation_timestamp: todo!(),
            // deletion_grace_period_seconds: todo!(),
            // deletion_timestamp: todo!(),
            // finalizers: todo!(),
            // generate_name: todo!(),
            // generation: todo!(),
            // labels: todo!(),
            // managed_fields: todo!(),
            // namespace: todo!(),
            // owner_references: todo!(),
            // resource_version: todo!(),
            // self_link: todo!(),
            // uid: todo!(),
            ..metav1::ObjectMeta::default()
        };

        Self {
            metadata: Some(metadata),
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
