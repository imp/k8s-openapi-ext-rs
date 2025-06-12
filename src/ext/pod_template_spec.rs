use super::*;

pub trait PodTemplateSpecExt: Sized {
    fn new() -> Self;

    fn label(self, key: impl ToString, value: impl ToString) -> Self {
        self.labels([(key, value)])
    }

    fn labels(self, labels: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self;

    fn annotation(self, key: impl ToString, value: impl ToString) -> Self {
        self.annotations([(key, value)])
    }

    fn annotations(
        self,
        annotations: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    /// Set recommended label 'app.kubernetes.io/name'
    ///
    fn app_name(self, name: impl ToString) -> Self {
        self.labels([(label::APP_NAME, name)])
    }

    /// Set recommended label 'app.kubernetes.io/instance'
    ///
    fn app_instance(self, instance: impl ToString) -> Self {
        self.labels([(label::APP_INSTANCE, instance)])
    }

    /// Set recommended label 'app.kubernetes.io/version'
    ///
    fn app_version(self, version: impl ToString) -> Self {
        self.labels([(label::APP_VERSION, version)])
    }

    /// Set recommended label 'app.kubernetes.io/component'
    ///
    fn app_component(self, component: impl ToString) -> Self {
        self.labels([(label::APP_COMPONENT, component)])
    }

    /// Set recommended label 'app.kubernetes.io/part-of'
    ///
    fn app_part_of(self, part_of: impl ToString) -> Self {
        self.labels([(label::APP_PART_OF, part_of)])
    }

    /// Set recommended label 'app.kubernetes.io/managed-by'
    ///
    fn app_managed_by(self, managed_by: impl ToString) -> Self {
        self.labels([(label::APP_MANAGED_BY, managed_by)])
    }

    fn pod_spec(self, spec: corev1::PodSpec) -> Self;
}

impl PodTemplateSpecExt for corev1::PodTemplateSpec {
    fn new() -> Self {
        Self {
            metadata: None,
            spec: None,
        }
    }

    fn labels(mut self, labels: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self {
        let labels = labels
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()));
        self.metadata
            .get_or_insert_default()
            .labels
            .get_or_insert_default()
            .extend(labels);
        self
    }

    fn annotations(
        mut self,
        annotations: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        let annotations = annotations
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()));
        self.metadata
            .get_or_insert_default()
            .annotations
            .get_or_insert_default()
            .extend(annotations);
        self
    }

    fn pod_spec(self, spec: corev1::PodSpec) -> Self {
        Self {
            spec: Some(spec),
            ..self
        }
    }
}

impl HasSpec for corev1::PodTemplateSpec {
    type Spec = corev1::PodSpec;

    fn spec_mut(&mut self) -> &mut Self::Spec {
        self.spec.get_or_insert_default()
    }
}
