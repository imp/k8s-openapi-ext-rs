use super::*;

/// Builders for `corev1::EnvVar` objects
pub trait EnvVarExt {
    /// Set an env var `name` with a given `value`
    fn value(name: impl ToString, value: impl ToString) -> Self;
    /// Set an env var `name` that points to a pod's name
    fn metadata_name(name: impl ToString) -> Self;
    fn metadata_namespace(name: impl ToString) -> Self;
    fn metadata_uid(name: impl ToString) -> Self;
    // metadata.labels\['\<KEY\>'\], metadata.annotations\['\<KEY\>'\],
    /// Set an env var `name` that points to the node which this pod was requested to scheduled on
    fn spec_nodename(name: impl ToString) -> Self;
    /// Set an env var `name` that points to the requested ServiceAccountName for this pod
    fn spec_service_account_name(name: impl ToString) -> Self;
    fn status_host_ip(name: impl ToString) -> Self;
    fn status_pod_ip(name: impl ToString) -> Self;
    fn status_pod_ips(name: impl ToString) -> Self;
    /// Set an env var `name` that points to the selected field in the pod
    fn field_ref(name: impl ToString, field_path: impl ToString) -> Self;
}

impl EnvVarExt for corev1::EnvVar {
    fn value(name: impl ToString, value: impl ToString) -> Self {
        let name = name.to_string();
        let value = Some(value.to_string());
        Self {
            name,
            value,
            value_from: None,
        }
    }

    fn metadata_name(name: impl ToString) -> Self {
        Self::field_ref(name, "metadata.name")
    }

    fn metadata_namespace(name: impl ToString) -> Self {
        Self::field_ref(name, "metadata.namespace")
    }

    fn metadata_uid(name: impl ToString) -> Self {
        Self::field_ref(name, "metadata.uid")
    }

    fn spec_nodename(name: impl ToString) -> Self {
        Self::field_ref(name, "spec.nodeName")
    }

    fn spec_service_account_name(name: impl ToString) -> Self {
        Self::field_ref(name, "spec.serviceAccountName")
    }

    fn status_host_ip(name: impl ToString) -> Self {
        Self::field_ref(name, "status.hostIP")
    }
    fn status_pod_ip(name: impl ToString) -> Self {
        Self::field_ref(name, "status.podIP")
    }
    fn status_pod_ips(name: impl ToString) -> Self {
        Self::field_ref(name, "status.podIPs")
    }

    fn field_ref(name: impl ToString, field_path: impl ToString) -> Self {
        let name = name.to_string();
        let field_ref = corev1::ObjectFieldSelector {
            api_version: None,
            field_path: field_path.to_string(),
        };
        let source = corev1::EnvVarSource {
            field_ref: Some(field_ref),
            ..default()
        };
        Self {
            name,
            value: None,
            value_from: Some(source),
        }
    }
}

pub trait ToEnvVar {
    fn to_envvar(&self) -> corev1::EnvVar;
}

impl ToEnvVar for corev1::EnvVar {
    fn to_envvar(&self) -> corev1::EnvVar {
        self.clone()
    }
}

impl<T, U> ToEnvVar for (T, U)
where
    T: fmt::Display,
    U: fmt::Display,
{
    fn to_envvar(&self) -> corev1::EnvVar {
        let (ref name, ref value) = *self;
        corev1::EnvVar::value(name, value)
    }
}

pub trait ToEnvFrom {
    fn to_envfrom(self) -> corev1::EnvFromSource;
}

impl ToEnvFrom for corev1::SecretEnvSource {
    fn to_envfrom(self) -> corev1::EnvFromSource {
        corev1::EnvFromSource {
            secret_ref: Some(self),
            ..default()
        }
    }
}

impl ToEnvFrom for corev1::ConfigMapEnvSource {
    fn to_envfrom(self) -> corev1::EnvFromSource {
        corev1::EnvFromSource {
            config_map_ref: Some(self),
            ..default()
        }
    }
}
