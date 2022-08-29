use super::*;

// // SecretTypeDockerConfigJSON contains a dockercfg file that follows the same format rules as ~/.docker/config.json
// //
// // Required fields:
// // - Secret.Data[".dockerconfigjson"] - a serialized ~/.docker/config.json file
// SecretTypeDockerConfigJSON SecretType = "kubernetes.io/dockerconfigjson"

// // DockerConfigJSONKey is the key of the required data for SecretTypeDockerConfigJson secrets
// DockerConfigJSONKey = ".dockerconfigjson"

const DOCKER_CONFIG_JSON_TYPE: &str = "kubernetes.io/dockerconfigjson";
const DOCKER_CONFIG_JSON_KEY: &str = ".dockerconfigjson";

const BASIC_AUTH_TYPE: &str = "kubernetes.io/basic-auth";
const BASIC_AUTH_USERNAME: &str = "username";
const BASIC_AUTH_PASSWORD: &str = "password";

const SSH_AUTH_TYPE: &str = "kubernetes.io/ssh-auth";
const SSH_AUTH_PRIVATE_KEY: &str = "ssh-privatekey";

pub trait SecretExt: super::ResourceBuilder + Sized {
    fn new(name: impl ToString) -> Self;

    fn immutable(self, yes: bool) -> Self;

    fn r#type(self, r#type: impl ToString) -> Self;

    fn data(self, data: impl IntoIterator<Item = (impl ToString, ByteString)>) -> Self;

    fn string_data(self, data: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self;

    /// Creates new image pull secret object
    ///
    fn image_pull_secret(
        name: impl ToString,
        registry: impl ToString,
        username: impl ToString,
        password: impl ToString,
    ) -> Self {
        let registry = registry.to_string();
        let username = username.to_string();
        let password = password.to_string();
        let auth = format!("{username}:{password}");
        let auth = base64::encode(auth);
        let config = format!(
            r#"{{"auths":{{"{registry}":{{"username":"{username}","password":"{password}","auth":"{auth}"}}}}}}"#
        );
        let data = [(DOCKER_CONFIG_JSON_KEY, config)];
        Self::new(name)
            .r#type(DOCKER_CONFIG_JSON_TYPE)
            .string_data(data)
    }

    /// Creates new basic authentication secret object
    ///
    fn basic_auth(name: impl ToString, username: impl ToString, password: impl ToString) -> Self {
        let data = [
            (BASIC_AUTH_USERNAME, username.to_string()),
            (BASIC_AUTH_PASSWORD, password.to_string()),
        ];
        Self::new(name).r#type(BASIC_AUTH_TYPE).string_data(data)
    }

    fn ssh_auth(name: impl ToString, private_key: impl ToString) -> Self {
        let data = [(SSH_AUTH_PRIVATE_KEY, private_key)];
        Self::new(name).r#type(SSH_AUTH_TYPE).string_data(data)
    }
}

pub trait SecretExt2: SecretExt {
    /// Creates new image pull secret object when you already have the .docker/config.json
    /// content extracted from some other source (i.e. secret)
    ///
    fn image_pull_secret(name: impl ToString, data: impl ToString) -> Self {
        let data = [(DOCKER_CONFIG_JSON_KEY, data)];
        Self::new(name)
            .r#type(DOCKER_CONFIG_JSON_TYPE)
            .string_data(data)
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

    fn data(self, data: impl IntoIterator<Item = (impl ToString, ByteString)>) -> Self {
        let data = data
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect();
        Self {
            data: Some(data),
            ..self
        }
    }

    fn string_data(self, data: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self {
        let data = data
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        Self {
            string_data: Some(data),
            ..self
        }
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
        // println!("{secret:#?}");
        let string_data = secret.string_data.unwrap_or_default();
        assert_eq!(string_data.len(), 1);
        let config: json::Value = json::from_str(&string_data[DOCKER_CONFIG_JSON_KEY]).unwrap();
        // println!("{config:#?}");
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
        assert_eq!(string_data[SSH_AUTH_PRIVATE_KEY].len(), 70);
    }
}
