use super::*;

pub trait StatefulSetExt {
    /// Create a new StatefulSet with the given name.
    fn new(name: impl ToString) -> Self;

    /// Set the spec for the StatefulSet.
    fn spec(self, spec: appsv1::StatefulSetSpec) -> Self;

    /// Set the number of replicas for the StatefulSet.
    fn replicas(self, replicas: i32) -> Self;

    /// Set the selector for the StatefulSet.
    fn selector(self, selector: metav1::LabelSelector) -> Self;

    /// Set the matchLabels for the StatefulSet's selector.
    fn match_labels(
        self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    /// Set the service name for the StatefulSet.
    fn service_name(self, service_name: impl ToString) -> Self;

    /// Set the pod template spec for the StatefulSet.
    fn template(self, template: corev1::PodTemplateSpec) -> Self;

    fn ordinals(self, ordinals: i32) -> Self;
}

impl StatefulSetExt for appsv1::StatefulSet {
    fn new(name: impl ToString) -> Self {
        let metadata = metadata(name);
        Self {
            metadata,
            // spec: todo!(),
            // status: todo!(),
            ..default()
        }
    }

    fn spec(self, spec: appsv1::StatefulSetSpec) -> Self {
        Self {
            spec: Some(spec),
            ..self
        }
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
        mut self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        self.spec_mut().selector = metav1::LabelSelector::match_labels(match_labels);
        self
    }

    fn service_name(mut self, service_name: impl ToString) -> Self {
        self.spec_mut()
            .service_name
            .replace(service_name.to_string());
        self
    }

    fn template(mut self, template: corev1::PodTemplateSpec) -> Self {
        self.spec_mut().template = template;
        self
    }

    fn ordinals(mut self, start: i32) -> Self {
        self.spec_mut().ordinals = Some(appsv1::StatefulSetOrdinals { start: Some(start) });
        self
    }
}

impl HasSpec for appsv1::StatefulSet {
    type Spec = appsv1::StatefulSetSpec;

    fn spec_mut(&mut self) -> &mut Self::Spec {
        self.spec.get_or_insert_default()
    }
}
