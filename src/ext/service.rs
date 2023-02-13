use super::*;

pub trait ServiceExt: super::ResourceBuilder {
    fn new(name: impl ToString) -> Self;
    fn cluster_ip(
        name: impl ToString,
        ports: impl IntoIterator<Item = corev1::ServicePort>,
    ) -> Self;
    fn node_port(name: impl ToString) -> Self;
    fn load_balancer(name: impl ToString) -> Self;
    fn external_name(name: impl ToString, external_name: impl ToString) -> Self;
    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    fn spec(self, spec: corev1::ServiceSpec) -> Self;

    fn selector(
        self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;
}

impl ServiceExt for corev1::Service {
    fn new(name: impl ToString) -> Self {
        let metadata = metadata(name);
        Self {
            metadata,
            // spec: todo!(),
            // status: todo!(),
            ..default()
        }
    }

    fn cluster_ip(
        name: impl ToString,
        ports: impl IntoIterator<Item = corev1::ServicePort>,
    ) -> Self {
        let mut service = Self::with_type(name, "ClusterIP");
        service
            .spec_mut()
            .ports
            .replace(ports.into_iter().collect());
        service
    }

    fn node_port(name: impl ToString) -> Self {
        Self::with_type(name, "NodePort")
    }

    fn load_balancer(name: impl ToString) -> Self {
        Self::with_type(name, "LoadBalancer")
    }

    fn external_name(name: impl ToString, external_name: impl ToString) -> Self {
        let mut service = Self::with_type(name, "ExternalName");
        service
            .spec_mut()
            .external_name
            .replace(external_name.to_string());
        service
    }

    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        Self::new(name).labels(labels)
    }

    fn spec(self, spec: corev1::ServiceSpec) -> Self {
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn selector(
        mut self,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        let labels = labels
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        self.spec_mut().selector.replace(labels);
        self
    }
}

trait ServiceExtPrivate {
    fn with_type(name: impl ToString, r#type: impl ToString) -> Self;
}

impl ServiceExtPrivate for corev1::Service {
    fn with_type(name: impl ToString, r#type: impl ToString) -> Self {
        let type_ = Some(r#type.to_string());
        let spec = corev1::ServiceSpec { type_, ..default() };
        Self::new(name).spec(spec)
    }
}

impl HasSpec for corev1::Service {
    type Spec = corev1::ServiceSpec;

    fn spec_mut(&mut self) -> &mut Self::Spec {
        self.spec.get_or_insert_with(default)
    }
}
