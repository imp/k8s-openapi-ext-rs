use super::*;

pub trait DeploymentExt: super::ResourceBuilder {
    fn new(name: impl ToString) -> Self;
    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    fn paused(self, yes: bool) -> Self;

    fn progress_deadline_seconds(self, seconds: i32) -> Self;

    fn replicas(self, replicas: i32) -> Self;

    fn revision_history_limit(self, limit: i32) -> Self;

    fn selector(self, selector: metav1::LabelSelector) -> Self;

    fn match_labels(
        self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    fn strategy(self, strategy: appsv1::DeploymentStrategy) -> Self;

    fn template(self, template: corev1::PodTemplateSpec) -> Self;

    fn pod(self, pod: corev1::PodSpec) -> Self;
}

impl DeploymentExt for appsv1::Deployment {
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

    fn paused(mut self, yes: bool) -> Self {
        self.spec_mut().paused.replace(yes);
        self
    }

    fn progress_deadline_seconds(mut self, seconds: i32) -> Self {
        self.spec_mut().progress_deadline_seconds.replace(seconds);
        self
    }

    fn replicas(mut self, replicas: i32) -> Self {
        self.spec_mut().replicas.replace(replicas);
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

    fn strategy(mut self, strategy: appsv1::DeploymentStrategy) -> Self {
        self.spec_mut().strategy.replace(strategy);
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

impl HasSpec for appsv1::Deployment {
    type Spec = appsv1::DeploymentSpec;

    fn spec_mut(&mut self) -> &mut Self::Spec {
        self.spec.get_or_insert_default()
    }
}
