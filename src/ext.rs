use std::collections::BTreeMap;

use super::*;

pub use cluster_role::ClusterRoleExt;
pub use cluster_role_binding::ClusterRoleBindingExt;
pub use configmap::ConfigMapExt;
pub use container::ContainerExt;
pub use daemon_set::DaemonSetExt;
pub use deployment::DeploymentExt;
pub use env::EnvVarExt;
pub use env::ToEnvFrom;
pub use env::ToEnvVar;
pub use job::JobExt;
pub use label_selector::LabelSelectorExt;
pub use namespace::NamespaceExt;
pub use pod_spec::PodSpecExt;
pub use pod_template_spec::PodTemplateSpecExt;
pub use policy_rule::PolicyRuleExt;
pub use probe::ProbeExt;
pub use role::RoleExt;
pub use role_binding::RoleBindingExt;
pub use role_ref::RoleRefExt;
pub use secret::SecretExt;
pub use secret::SecretExt2;
pub use secret_env_source::SecretEnvSourceExt;
pub use secret_volume_source::SecretVolumeSourceExt;
pub use service::ServiceExt;
pub use service_account::ServiceAccountExt;
pub use subject::SubjectExt;
pub use volume::VolumeExt;
pub use volume_mount::VolumeMountExt;

mod cluster_role;
mod cluster_role_binding;
mod configmap;
mod container;
mod daemon_set;
mod deployment;
mod env;
mod job;
mod label_selector;
mod namespace;
mod pod_spec;
mod pod_template_spec;
mod policy_rule;
mod probe;
mod role;
mod role_binding;
mod role_ref;
mod secret;
mod secret_env_source;
mod secret_volume_source;
mod service;
mod service_account;
mod subject;
mod volume;
mod volume_mount;

pub trait ResourceBuilder {
    fn metadata(name: impl ToString) -> metav1::ObjectMeta {
        let name = Some(name.to_string());
        metav1::ObjectMeta {
            name,
            ..metav1::ObjectMeta::default() // annotations: todo!(),
                                            // cluster_name: todo!(),
                                            // creation_timestamp: todo!(),
                                            // deletion_grace_period_seconds: todo!(),
                                            // deletion_timestamp: todo!(),
                                            // finalizers: todo!(),
                                            // generate_name: todo!(),
                                            // generation: todo!(),
                                            // labels: todo!(),
                                            // managed_fields: todo!(),
                                            // namespace: todo!(),
                                            // owner_references: todo!(),
                                            // resource_version: todo!(),
                                            // self_link: todo!(),
                                            // uid: todo!(),
        }
    }

    fn namespace(self, namespace: impl ToString) -> Self;
    fn owner(self, owner: metav1::OwnerReference) -> Self;
    fn labels(self, labels: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self;
    fn with_resource_version(self, resource_version: String) -> Self;
}

impl<T> ResourceBuilder for T
where
    T: openapi::Metadata<Ty = metav1::ObjectMeta>,
{
    fn namespace(mut self, namespace: impl ToString) -> Self {
        let namespace = Some(namespace.to_string());
        self.metadata_mut().namespace = namespace;
        self
    }

    fn owner(mut self, owner: metav1::OwnerReference) -> Self {
        self.metadata_mut()
            .owner_references
            .get_or_insert_with(Vec::new)
            .push(owner);
        self
    }

    fn labels(mut self, labels: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self {
        let labels = labels
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()));
        self.metadata_mut()
            .labels
            .get_or_insert_with(BTreeMap::new)
            .extend(labels);
        self
    }

    fn with_resource_version(mut self, resource_version: String) -> Self {
        self.metadata_mut().resource_version = Some(resource_version);
        self
    }
}
