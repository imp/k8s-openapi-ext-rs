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

    fn min_ready_seconds(mut self, seconds: i32) -> Self {
        self.spec_mut().min_ready_seconds.replace(seconds);
        self
    }

    fn replicas(mut self, replicas: i32) -> Self {
        self.spec_mut().replicas.replace(replicas);
        self
    }

    fn selector(mut self, selector: metav1::LabelSelector) -> Self {
        self.spec_mut().selector = selector;
        self
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

    fn template(mut self, template: corev1::PodTemplateSpec) -> Self {
        self.spec_mut().template.replace(template);
        self
    }

    fn pod_spec(mut self, pod_spec: corev1::PodSpec) -> Self {
        self.spec_mut()
            .template
            .get_or_insert_default()
            .spec
            .replace(pod_spec);
        self
    }
}

impl HasSpec for appsv1::ReplicaSet {
    type Spec = appsv1::ReplicaSetSpec;

    fn spec_mut(&mut self) -> &mut Self::Spec {
        self.spec.get_or_insert_default()
    }
}
