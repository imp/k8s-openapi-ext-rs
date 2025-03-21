use super::*;

pub trait DaemonSetExt: super::ResourceBuilder {
    fn new(name: impl ToString) -> Self;
    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    fn spec(self, spec: appsv1::DaemonSetSpec) -> Self;

    fn min_ready_seconds(self, seconds: i32) -> Self;

    fn revision_history_limit(self, limit: i32) -> Self;

    fn selector(self, selector: metav1::LabelSelector) -> Self;

    fn match_labels(
        self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    fn update_strategy(self, strategy: appsv1::DaemonSetUpdateStrategy) -> Self;

    fn template(self, template: corev1::PodTemplateSpec) -> Self;

    fn pod(self, pod: corev1::PodSpec) -> Self;
}

impl DaemonSetExt for appsv1::DaemonSet {
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

    fn spec(self, spec: appsv1::DaemonSetSpec) -> Self {
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn min_ready_seconds(mut self, seconds: i32) -> Self {
        self.spec_mut().min_ready_seconds.replace(seconds);
        self
    }

    fn revision_history_limit(mut self, limit: i32) -> Self {
        self.spec_mut().revision_history_limit.replace(limit);
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

    fn update_strategy(mut self, strategy: appsv1::DaemonSetUpdateStrategy) -> Self {
        self.spec_mut().update_strategy.replace(strategy);
        self
    }

    fn template(mut self, template: corev1::PodTemplateSpec) -> Self {
        self.spec_mut().template = template;
        self
    }

    fn pod(mut self, pod_spec: corev1::PodSpec) -> Self {
        self.spec_mut().template.spec.replace(pod_spec);
        self
    }
}

impl HasSpec for appsv1::DaemonSet {
    type Spec = appsv1::DaemonSetSpec;

    fn spec_mut(&mut self) -> &mut Self::Spec {
        self.spec.get_or_insert_default()
    }
}
