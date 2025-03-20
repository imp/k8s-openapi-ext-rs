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
#![warn(rust_2024_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

pub use k8s_openapi as openapi;

pub use openapi::api::admissionregistration::v1 as admissionregistrationv1;
pub use openapi::api::apps::v1 as appsv1;
pub use openapi::api::authentication::v1 as authenticationv1;
pub use openapi::api::authorization::v1 as authorizationv1;
pub use openapi::api::autoscaling::v1 as autoscalingv1;
pub use openapi::api::autoscaling::v2 as autoscalingv2;
pub use openapi::api::batch::v1 as batchv1;
pub use openapi::api::certificates::v1 as certificatesv1;
pub use openapi::api::coordination::v1 as coordinationv1;
pub use openapi::api::core::v1 as corev1;
pub use openapi::api::events::v1 as eventsv1;
pub use openapi::api::node::v1 as nodev1;
pub use openapi::api::policy::v1 as policyv1;
pub use openapi::api::rbac::v1 as rbacv1;
pub use openapi::api::scheduling::v1 as schedulingv1;
pub use openapi::api::storage::v1 as storagev1;
pub use openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiextensionsv1;
pub use openapi::apimachinery::pkg::api::resource;
pub use openapi::apimachinery::pkg::apis::meta::v1 as metav1;
pub use openapi::apimachinery::pkg::util::intstr;
pub use openapi::kube_aggregator::pkg::apis::apiregistration::v1 as apiregistrationv1;
pub use openapi::ByteString;
pub use openapi::Metadata;
pub use openapi::Resource;

openapi::k8s_if_ge_1_29! {
pub use openapi::api::flowcontrol::v1 as flowcontrolv1;
}

pub use ext::ClusterRoleBindingExt;
pub use ext::ClusterRoleExt;
pub use ext::ConfigMapExt;
pub use ext::ConfigMapVolumeSourceExt;
pub use ext::ContainerExt;
pub use ext::CronJobExt;
pub use ext::DaemonSetExt;
pub use ext::DeploymentExt;
pub use ext::EnvVarExt;
pub use ext::HorizontalPodAutoscalerExt;
pub use ext::JobExt;
pub use ext::LabelSelectorExt;
pub use ext::LocalObjectReferenceExt;
pub use ext::MetricExt;
pub use ext::NamespaceExt;
pub use ext::NodeExt;
pub use ext::OwnerReferenceExt;
pub use ext::PodExt;
pub use ext::PodSpecExt;
pub use ext::PodTemplateSpecExt;
pub use ext::PolicyRuleExt;
pub use ext::ProbeExt;
pub use ext::ReplicaSetExt;
pub use ext::ResourceBuilder;
pub use ext::RoleBindingExt;
pub use ext::RoleExt;
pub use ext::RoleRefExt;
pub use ext::SecretEnvSourceExt;
pub use ext::SecretExt;
pub use ext::SecretExt2;
pub use ext::SecretReferenceExt;
pub use ext::SecretVolumeSourceExt;
pub use ext::ServiceAccountExt;
pub use ext::ServiceExt;
pub use ext::ServicePortExt;
pub use ext::StorageClassExt;
pub use ext::SubjectExt;
pub use ext::TaintExt;
pub use ext::TimeExt;
pub use ext::ToEnvFrom;
pub use ext::ToEnvVar;
pub use ext::TolerationBuilder;
pub use ext::TolerationExt;
pub use ext::TypedObjectReferenceExt;
pub use ext::VolumeExt;
pub use ext::VolumeMountExt;
pub use get::ConfigMapGetExt;
pub use get::ContainerGetExt;
pub use get::ContainerStatusGetExt;
pub use get::DeploymentGetExt;
pub use get::EphemeralContainerGetExt;
pub use get::NamespaceGetExt;
pub use get::PodConditionGetExt;
pub use get::PodGetExt;
pub use get::ReplicaSetConditionGetExt;
pub use get::ReplicaSetGetExt;

mod ext;
mod get;

pub mod namespace {
    pub const DEFAULT: &str = "default";
    pub const KUBE_PUBLIC: &str = "kube-public";
    pub const KUBE_SYSTEM: &str = "kube-system";
}

pub mod label {
    pub const APP_NAME: &str = "app.kubernetes.io/name";
    pub const APP_INSTANCE: &str = "app.kubernetes.io/instance";
    pub const APP_VERSION: &str = "app.kubernetes.io/version";
    pub const APP_COMPONENT: &str = "app.kubernetes.io/component";
    pub const APP_PART_OF: &str = "app.kubernetes.io/part-of";
    pub const APP_MANAGED_BY: &str = "app.kubernetes.io/managed-by";
    pub const DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY: &str = "pod-template-hash";
}

pub fn typed_ref(
    object: Option<&corev1::ObjectReference>,
) -> Option<corev1::TypedLocalObjectReference> {
    let object = object?;
    let kind = object.kind.as_ref()?.clone();
    let name = object.name.as_ref()?.clone();
    let typed = corev1::TypedLocalObjectReference {
        kind,
        name,
        ..default()
    };
    Some(typed)
}

pub fn local_ref(object: &corev1::ObjectReference) -> Option<corev1::LocalObjectReference> {
    object.name.as_ref().map(corev1::LocalObjectReference::new)
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

fn default<T: Default>() -> T {
    T::default()
}
