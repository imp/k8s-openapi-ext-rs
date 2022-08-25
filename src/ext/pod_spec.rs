use super::*;

pub trait PodSpecExt {
    fn container(container: corev1::Container) -> Self;
    fn containers(containers: impl IntoIterator<Item = corev1::Container>) -> Self;

    fn service_account_name(self, name: impl ToString) -> Self;

    fn image_pull_secret(self, name: impl ToString) -> Self;

    fn volumes(self, volumes: impl IntoIterator<Item = corev1::Volume>) -> Self;
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
            ..Self::default()
        }
    }

    fn containers(containers: impl IntoIterator<Item = corev1::Container>) -> Self {
        let containers = Vec::from_iter(containers);
        Self {
            containers,
            ..Self::default()
        }
    }

    fn service_account_name(self, name: impl ToString) -> Self {
        let service_account_name = Some(name.to_string());
        Self {
            service_account_name,
            ..self
        }
    }

    fn image_pull_secret(self, name: impl ToString) -> Self {
        let image_pull_secret_name = Some(name.to_string());
        Self {
            image_pull_secrets: Some(vec![corev1::LocalObjectReference {
                name: image_pull_secret_name,
            }]),
            ..self
        }
    }

    fn volumes(self, volumes: impl IntoIterator<Item = corev1::Volume>) -> Self {
        let volumes = Some(volumes.into_iter().collect());
        Self { volumes, ..self }
    }
}
