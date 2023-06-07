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
pub use local_object_reference::LocalObjectReferenceExt;
pub use namespace::NamespaceExt;
pub use pod_spec::PodSpecExt;
pub use pod_template_spec::PodTemplateSpecExt;
pub use policy_rule::PolicyRuleExt;
pub use probe::ProbeExt;
pub use replica_set::ReplicaSetExt;
pub use role::RoleExt;
pub use role_binding::RoleBindingExt;
pub use role_ref::RoleRefExt;
pub use secret::SecretExt;
pub use secret::SecretExt2;
pub use secret_env_source::SecretEnvSourceExt;
pub use secret_volume_source::SecretVolumeSourceExt;
pub use service::ServiceExt;
pub use service_account::ServiceAccountExt;
pub use service_port::ServicePortExt;
pub use storage_class::StorageClassExt;
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
mod local_object_reference;
mod namespace;
mod pod_spec;
mod pod_template_spec;
mod policy_rule;
mod probe;
mod replica_set;
mod role;
mod role_binding;
mod role_ref;
mod secret;
mod secret_env_source;
mod secret_volume_source;
mod service;
mod service_account;
mod service_port;
mod storage_class;
mod subject;
mod volume;
mod volume_mount;

pub trait ResourceBuilder: Sized {
    fn metadata(name: impl ToString) -> metav1::ObjectMeta {
        metadata(name)
    }

    fn namespace(self, namespace: impl ToString) -> Self;
    fn owner(self, owner: metav1::OwnerReference) -> Self;
    fn labels(self, labels: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self;
    fn with_resource_version(self, resource_version: String) -> Self;

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
            .get_or_insert_with(default)
            .push(owner);
        self
    }

    fn labels(mut self, labels: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self {
        let labels = labels
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()));
        self.metadata_mut()
            .labels
            .get_or_insert_with(default)
            .extend(labels);
        self
    }

    fn with_resource_version(mut self, resource_version: String) -> Self {
        self.metadata_mut().resource_version = Some(resource_version);
        self
    }
}

fn metadata(name: impl ToString) -> metav1::ObjectMeta {
    let name = Some(name.to_string());
    metav1::ObjectMeta {
        name,
        // annotations: todo!(),
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
        ..default()
    }
}

trait HasSpec {
    type Spec;
    fn spec_mut(&mut self) -> &mut Self::Spec;
}
