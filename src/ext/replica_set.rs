use super::*;

pub trait ReplicaSetExt: super::ResourceBuilder {
    fn new(name: impl ToString) -> Self;
    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    fn spec(self, spec: appsv1::ReplicaSetSpec) -> Self;

    fn min_ready_seconds(self, seconds: i32) -> Self;

    fn replicas(self, replicas: i32) -> Self;

    fn selector(self, selector: metav1::LabelSelector) -> Self;

    fn match_labels(
        self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    fn template(self, template: corev1::PodTemplateSpec) -> Self;

    fn pod_spec(self, pod: corev1::PodSpec) -> Self;
}

impl ReplicaSetExt for appsv1::ReplicaSet {
    fn new(name: impl ToString) -> Self {
        let metadata = metadata(name);
        Self {
            metadata,
            // spec: todo!(),
            // status: todo!(),
            ..default()
        }
    }

    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        Self::new(name).labels(labels)
    }

    fn spec(self, spec: appsv1::ReplicaSetSpec) -> Self {
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn min_ready_seconds(self, seconds: i32) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.min_ready_seconds = Some(seconds);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn replicas(self, replicas: i32) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.replicas = Some(replicas);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn selector(self, selector: metav1::LabelSelector) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.selector = selector;
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn match_labels(
        self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.selector = spec.selector.match_labels(match_labels);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn template(self, template: corev1::PodTemplateSpec) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.template = Some(template);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn pod_spec(self, pod_spec: corev1::PodSpec) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.template
            .get_or_insert_with(corev1::PodTemplateSpec::default)
            .spec = Some(pod_spec);
        Self {
            spec: Some(spec),
            ..self
        }
    }
}
