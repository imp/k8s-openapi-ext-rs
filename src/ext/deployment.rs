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
        let metadata = Self::metadata(name);
        Self {
            metadata,
            // spec: todo!(),
            // status: todo!(),
            ..Self::default()
        }
    }

    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        Self::new(name).labels(labels)
    }

    fn paused(self, yes: bool) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.paused = Some(yes);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn progress_deadline_seconds(self, seconds: i32) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.progress_deadline_seconds = Some(seconds);
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

    fn revision_history_limit(self, limit: i32) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.revision_history_limit = Some(limit);
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

    fn strategy(self, strategy: appsv1::DeploymentStrategy) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.strategy = Some(strategy);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn template(self, template: corev1::PodTemplateSpec) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.template = template;
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn pod(self, pod_spec: corev1::PodSpec) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.template.spec = Some(pod_spec);
        Self {
            spec: Some(spec),
            ..self
        }
    }
}
