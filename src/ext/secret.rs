use base64::Engine;

use super::*;

pub trait SecretExt: super::ResourceBuilder + Sized {
    /// SecretTypeOpaque is the default; arbitrary user-defined data
    const SECRET_TYPE_OPAQUE: &str = "Opaque";

    /// SecretTypeServiceAccountToken contains a token that identifies a service account to the API
    ///
    /// Required fields:
    /// - Secret.Annotations["kubernetes.io/service-account.name"] - the name of the ServiceAccount the token identifies
    /// - Secret.Annotations["kubernetes.io/service-account.uid"] - the UID of the ServiceAccount the token identifies
    /// - Secret.Data["token"] - a token that identifies the service account to the API
    const SECRET_TYPE_SERVICE_ACCOUNT_TOKEN: &str = "kubernetes.io/service-account-token";

    /// ServiceAccountNameKey is the key of the required annotation for SecretTypeServiceAccountToken secrets
    const SERVICE_ACCOUNT_NAME_KEY: &str = "kubernetes.io/service-account.name";
    /// ServiceAccountUIDKey is the key of the required annotation for SecretTypeServiceAccountToken secrets
    const SERVICE_ACCOUNT_UID_KEY: &str = "kubernetes.io/service-account.uid";
    /// ServiceAccountTokenKey is the key of the required data for SecretTypeServiceAccountToken secrets
    const SERVICE_ACCOUNT_TOKEN_KEY: &str = "token";
    /// ServiceAccountKubeconfigKey is the key of the optional kubeconfig data for SecretTypeServiceAccountToken secrets
    const SERVICE_ACCOUNT_KUBECONFIG_KEY: &str = "kubernetes.kubeconfig";
    /// ServiceAccountRootCAKey is the key of the optional root certificate authority for SecretTypeServiceAccountToken secrets
    const SERVICE_ACCOUNT_ROOTCA_KEY: &str = "ca.crt";
    /// ServiceAccountNamespaceKey is the key of the optional namespace to use as the default for namespaced API calls
    const SERVICE_ACCOUNT_NAMESPACE_KEY: &str = "namespace";

    /// SecretTypeDockercfg contains a dockercfg file that follows the same format rules as ~/.dockercfg
    ///
    /// Required fields:
    /// - Secret.Data[".dockercfg"] - a serialized ~/.dockercfg file
    const SECRET_TYPE_DOCKERCFG: &str = "kubernetes.io/dockercfg";

    /// DockerConfigKey is the key of the required data for SecretTypeDockercfg secrets
    const DOCKER_CONFIG_KEY: &str = ".dockercfg";

    /// SecretTypeDockerConfigJSON contains a dockercfg file that follows the same format rules as ~/.docker/config.json
    ///
    /// Required fields:
    /// - Secret.Data[".dockerconfigjson"] - a serialized ~/.docker/config.json file
    const SECRET_TYPE_DOCKER_CONFIG_JSON: &str = "kubernetes.io/dockerconfigjson";
    /// DockerConfigJSONKey is the key of the required data for SecretTypeDockerConfigJson secrets
    const DOCKER_CONFIG_JSON_KEY: &str = ".dockerconfigjson";

    /// SecretTypeBasicAuth contains data needed for basic authentication.
    ///
    /// Required at least one of fields:
    /// - Secret.Data["username"] - username used for authentication
    /// - Secret.Data["password"] - password or token needed for authentication
    const SECRET_TYPE_BASIC_AUTH: &str = "kubernetes.io/basic-auth";

    /// BasicAuthUsernameKey is the key of the username for SecretTypeBasicAuth secrets
    const BASIC_AUTH_USERNAME_KEY: &str = "username";
    /// BasicAuthPasswordKey is the key of the password or token for SecretTypeBasicAuth secrets
    const BASIC_AUTH_PASSWORD_KEY: &str = "password";

    /// SecretTypeSSHAuth contains data needed for SSH authentication.
    ///
    /// Required field:
    /// - Secret.Data["ssh-privatekey"] - private SSH key needed for authentication
    const SECRET_TYPE_SSH_AUTH: &str = "kubernetes.io/ssh-auth";

    /// SSHAuthPrivateKey is the key of the required SSH private key for SecretTypeSSHAuth secrets
    const SSH_AUTH_PRIVATE_KEY: &str = "ssh-privatekey";

    /// SecretTypeTLS contains information about a TLS client or server secret. It
    /// is primarily used with TLS termination of the Ingress resource, but may be
    /// used in other types.
    ///
    /// Required fields:
    /// - Secret.Data["tls.key"] - TLS private key.
    ///   Secret.Data["tls.crt"] - TLS certificate.
    const SECRET_TYPE_TLS: &str = "kubernetes.io/tls";

    /// TLSCertKey is the key for tls certificates in a TLS secret.
    const TLS_CERT_KEY: &str = "tls.crt";
    /// TLSPrivateKeyKey is the key for the private key field in a TLS secret.
    const TLS_PRIVATE_KEY_KEY: &str = "tls.key";

    /// SecretTypeBootstrapToken is used during the automated bootstrap process (first
    /// implemented by kubeadm). It stores tokens that are used to sign well known
    /// ConfigMaps. They are used for authn.
    const SECRET_TYPE_BOOTSTRAP_TOKEN: &str = "bootstrap.kubernetes.io/token";

    /// Creates new empty `corev1::Secret` object
    fn new(name: impl ToString) -> Self;

    /// Set immutability field
    fn immutable(self, yes: bool) -> Self;

    fn r#type(self, r#type: impl ToString) -> Self;

    fn data(self, data: impl IntoIterator<Item = (impl ToString, ByteString)>) -> Self;

    fn string_data(self, data: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self;

    /// Create new "Opaque" secret object with no data
    fn opaque(name: impl ToString) -> Self {
        Self::new(name).r#type(Self::SECRET_TYPE_OPAQUE)
    }

    /// Create "kubernetes.io/dockerconfigjson" secret with `config` as plain text
    /// (i.e. string_data field)
    fn docker_config_json(name: impl ToString, config: impl ToString) -> Self {
        let data = [(Self::DOCKER_CONFIG_JSON_KEY, config)];
        Self::new(name)
            .r#type(Self::SECRET_TYPE_DOCKER_CONFIG_JSON)
            .string_data(data)
    }

    /// Create "kubernetes.io/dockerconfigjson" secret with `config` as base64 encoded text
    /// (i.e. data field)
    fn docker_config_json_base64_encoded(name: impl ToString, config: ByteString) -> Self {
        let data = [(Self::DOCKER_CONFIG_JSON_KEY, config)];
        Self::new(name)
            .r#type(Self::SECRET_TYPE_DOCKER_CONFIG_JSON)
            .data(data)
    }

    /// Creates new basic authentication secret object
    fn basic_auth(name: impl ToString, username: impl ToString, password: impl ToString) -> Self {
        let data = [
            (Self::BASIC_AUTH_USERNAME_KEY, username.to_string()),
            (Self::BASIC_AUTH_PASSWORD_KEY, password.to_string()),
        ];
        Self::new(name)
            .r#type(Self::SECRET_TYPE_BASIC_AUTH)
            .string_data(data)
    }

    /// Creates new ssh auth secret object
    fn ssh_auth(name: impl ToString, private_key: impl ToString) -> Self {
        let data = [(Self::SSH_AUTH_PRIVATE_KEY, private_key)];
        Self::new(name)
            .r#type(Self::SECRET_TYPE_SSH_AUTH)
            .string_data(data)
    }

    /// Creates new image pull secret object
    fn image_pull_secret(
        name: impl ToString,
        registry: impl ToString,
        username: impl ToString,
        password: impl ToString,
    ) -> Self {
        let registry = registry.to_string();
        let username = username.to_string();
        let password = password.to_string();
        let auth = base64::prelude::BASE64_STANDARD.encode(format!("{username}:{password}"));
        let config = format!(
            r#"{{"auths":{{"{registry}":{{"username":"{username}","password":"{password}","auth":"{auth}"}}}}}}"#
        );
        Self::docker_config_json(name, config)
    }
}

pub trait SecretExt2: SecretExt {
    /// Creates new image pull secret object when you already have the .docker/config.json
    /// content extracted from some other source (i.e. secret)
    #[deprecated(since = "0.0.54", note = "use SecretExt::docker_config_json instead")]
    fn image_pull_secret(name: impl ToString, config: impl ToString) -> Self {
        Self::docker_config_json(name, config)
    }
}

impl SecretExt for corev1::Secret {
    fn new(name: impl ToString) -> Self {
        let metadata = metadata(name);
        Self {
            metadata,
            // immutable: todo!(),
            // data: todo!(),
            // string_data: todo!(),
            // type_: todo!(),
            ..default()
        }
    }

    fn immutable(self, yes: bool) -> Self {
        let immutable = Some(yes);
        Self { immutable, ..self }
    }

    fn r#type(self, r#type: impl ToString) -> Self {
        let type_ = Some(r#type.to_string());
        Self { type_, ..self }
    }

    fn data(mut self, data: impl IntoIterator<Item = (impl ToString, ByteString)>) -> Self {
        let iter = data
            .into_iter()
            .map(|(key, value)| (key.to_string(), value));
        self.data.get_or_insert_default().extend(iter);
        self
    }

    fn string_data(
        mut self,
        data: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        let iter = data
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()));
        self.string_data.get_or_insert_default().extend(iter);
        self
    }
}

impl SecretExt2 for corev1::Secret {}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json as json;

    #[test]
    fn image_pull_secret() {
        let secret = <corev1::Secret as SecretExt>::image_pull_secret(
            "name", "registry", "username", "password",
        );
        let string_data = secret.string_data.unwrap_or_default();
        assert_eq!(string_data.len(), 1);
        let config: json::Value =
            json::from_str(&string_data[corev1::Secret::DOCKER_CONFIG_JSON_KEY]).unwrap();
        assert!(config.is_object());
    }

    #[test]
    fn ssh_auth() {
        let secret = corev1::Secret::ssh_auth(
            "name",
            "KGpwKaqlGas+LaAqdwdfAAAEEAAAAzAAAAC3NzaC1lZDI1NTE5AAAAIClTFvhvwp1UH25b",
        );
        let string_data = secret.string_data.unwrap_or_default();
        assert_eq!(string_data.len(), 1);
        assert_eq!(string_data[corev1::Secret::SSH_AUTH_PRIVATE_KEY].len(), 70);
    }
}
