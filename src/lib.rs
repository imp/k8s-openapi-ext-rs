#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

pub use k8s_openapi as openapi;

pub use openapi::api::apps::v1 as appsv1;
pub use openapi::api::batch::v1 as batchv1;
pub use openapi::api::core::v1 as corev1;
pub use openapi::api::events::v1 as eventsv1;
pub use openapi::api::rbac::v1 as rbacv1;
pub use openapi::api::storage::v1 as storagev1;
pub use openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiextensionsv1;
pub use openapi::apimachinery::pkg::apis::meta::v1 as metav1;
pub use openapi::apimachinery::pkg::util::intstr;
pub use openapi::ByteString;
pub use openapi::Resource;

pub use ext::ClusterRoleBindingExt;
pub use ext::ClusterRoleExt;
pub use ext::ConfigMapExt;
pub use ext::ContainerExt;
pub use ext::DaemonSetExt;
pub use ext::DeploymentExt;
pub use ext::EnvVarExt;
pub use ext::JobExt;
pub use ext::LabelSelectorExt;
pub use ext::NamespaceExt;
pub use ext::PodSpecExt;
pub use ext::PodTemplateSpecExt;
pub use ext::PolicyRuleExt;
pub use ext::ProbeExt;
pub use ext::ResourceBuilder;
pub use ext::RoleBindingExt;
pub use ext::RoleExt;
pub use ext::RoleRefExt;
pub use ext::SecretEnvSourceExt;
pub use ext::SecretExt;
pub use ext::SecretVolumeSourceExt;
pub use ext::ServiceAccountExt;
pub use ext::ServiceExt;
pub use ext::SubjectExt;
pub use ext::ToEnvFrom;
pub use ext::ToEnvVar;
pub use ext::VolumeExt;
pub use ext::VolumeMountExt;

mod ext;

pub fn typed_ref(
    object: Option<&corev1::ObjectReference>,
) -> Option<corev1::TypedLocalObjectReference> {
    let object = object?;
    let kind = object.kind.as_ref()?.clone();
    let name = object.name.as_ref()?.clone();
    let typed = corev1::TypedLocalObjectReference {
        kind,
        name,
        ..corev1::TypedLocalObjectReference::default()
    };
    Some(typed)
}

pub fn owner_reference(
    owner: corev1::ObjectReference,
    is_controller: bool,
    block_owner_deletion: bool,
) -> Option<metav1::OwnerReference> {
    Some(metav1::OwnerReference {
        block_owner_deletion: Some(block_owner_deletion),
        controller: Some(is_controller),
        api_version: owner.api_version?,
        kind: owner.kind?,
        name: owner.name?,
        uid: owner.uid?,
    })
}
