use super::*;

pub trait PodSpecExt {
    /// Build `corev1::PodSpec` with this container
    ///
    fn container(container: corev1::Container) -> Self;

    /// Build `corev1::PodSpec` with these containers
    ///
    fn containers(containers: impl IntoIterator<Item = corev1::Container>) -> Self;

    /// Set service account name
    ///
    fn service_account_name(self, name: impl ToString) -> Self;

    /// Add image pull secret
    ///
    fn image_pull_secret(self, name: impl ToString) -> Self;

    /// Add `volumes`
    ///
    fn volumes(self, volumes: impl IntoIterator<Item = corev1::Volume>) -> Self;

    /// Add node selector
    ///
    fn node_selector(
        self,
        node_selector: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    /// Set affinity
    ///
    fn affinity(self, affinity: impl Into<Option<corev1::Affinity>>) -> Self;

    /// Add tolerations
    ///
    fn tolerations(self, tolerations: impl IntoIterator<Item = corev1::Toleration>) -> Self;
}

impl PodSpecExt for corev1::PodSpec {
    fn container(container: corev1::Container) -> Self {
        let containers = vec![container];
        Self {
            containers,
            // active_deadline_seconds: todo!(),
            // affinity: todo!(),
            // automount_service_account_token: todo!(),
            // dns_config: todo!(),
            // dns_policy: todo!(),
            // enable_service_links: todo!(),
            // ephemeral_containers: todo!(),
            // host_aliases: todo!(),
            // host_ipc: todo!(),
            // host_network: todo!(),
            // host_pid: todo!(),
            // hostname: todo!(),
            // image_pull_secrets: todo!(),
            // init_containers: todo!(),
            // node_name: todo!(),
            // node_selector: todo!(),
            // overhead: todo!(),
            // preemption_policy: todo!(),
            // priority: todo!(),
            // priority_class_name: todo!(),
            // readiness_gates: todo!(),
            // restart_policy: todo!(),
            // runtime_class_name: todo!(),
            // scheduler_name: todo!(),
            // security_context: todo!(),
            // service_account: todo!(),
            // service_account_name: todo!(),
            // set_hostname_as_fqdn: todo!(),
            // share_process_namespace: todo!(),
            // subdomain: todo!(),
            // termination_grace_period_seconds: todo!(),
            // tolerations: todo!(),
            // topology_spread_constraints: todo!(),
            // volumes: todo!(),
            ..default()
        }
    }

    fn containers(containers: impl IntoIterator<Item = corev1::Container>) -> Self {
        let containers = Vec::from_iter(containers);
        Self {
            containers,
            ..default()
        }
    }

    fn service_account_name(self, name: impl ToString) -> Self {
        let service_account_name = Some(name.to_string());
        Self {
            service_account_name,
            ..self
        }
    }

    fn image_pull_secret(mut self, name: impl ToString) -> Self {
        let secret = corev1::LocalObjectReference::new(name);
        self.image_pull_secrets.get_or_insert_default().push(secret);
        self
    }

    fn volumes(mut self, volumes: impl IntoIterator<Item = corev1::Volume>) -> Self {
        self.volumes.get_or_insert_default().extend(volumes);
        self
    }

    fn node_selector(
        mut self,
        node_selector: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        let node_selector = node_selector
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()));
        self.node_selector
            .get_or_insert_default()
            .extend(node_selector);
        self
    }

    fn affinity(self, affinity: impl Into<Option<corev1::Affinity>>) -> Self {
        let affinity = affinity.into();
        Self { affinity, ..self }
    }

    fn tolerations(mut self, tolerations: impl IntoIterator<Item = corev1::Toleration>) -> Self {
        self.tolerations.get_or_insert_default().extend(tolerations);
        self
    }
}
