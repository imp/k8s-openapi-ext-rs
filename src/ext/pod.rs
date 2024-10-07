use super::*;

pub trait PodExt {
    /// Creats new `corev1::Pod` object
    ///
    fn new(name: impl ToString) -> Self;

    /// Creates new `corev1::Pod` object with given `labels`
    ///
    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    /// Sets `spec`
    ///
    fn spec(self, spec: corev1::PodSpec) -> Self;

    /// Sets this pod container
    ///
    fn container(self, container: corev1::Container) -> Self;

    /// Sets this pod containers
    ///
    fn containers(self, containers: impl IntoIterator<Item = corev1::Container>) -> Self;
}

impl PodExt for corev1::Pod {
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

    fn spec(self, spec: corev1::PodSpec) -> Self {
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn container(mut self, container: corev1::Container) -> Self {
        self.spec_mut().containers = vec![container];
        self
    }

    fn containers(mut self, containers: impl IntoIterator<Item = corev1::Container>) -> Self {
        self.spec_mut().containers = containers.into_iter().collect();
        self
    }
}

impl HasSpec for corev1::Pod {
    type Spec = corev1::PodSpec;

    fn spec_mut(&mut self) -> &mut Self::Spec {
        self.spec.get_or_insert_with(default)
    }
}
