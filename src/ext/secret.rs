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

pub trait SecretExt: super::ResourceBuilder + Sized {
    fn new(name: impl ToString) -> Self;

    fn immutable(self, yes: bool) -> Self;

    fn r#type(self, r#type: impl ToString) -> Self;

    fn data(self, data: impl IntoIterator<Item = (impl ToString, ByteString)>) -> Self;

    fn string_data(self, data: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self;
    fn image_pull_secret(name: impl ToString, data: impl ToString) -> Self {
        let data = Some((DOCKER_CONFIG_JSON_KEY.to_string(), data.to_string()));
        Self::new(name)
            .r#type(DOCKER_CONFIG_JSON_TYPE)
            .string_data(data)
    }
}

impl SecretExt for corev1::Secret {
    fn new(name: impl ToString) -> Self {
        let metadata = Self::metadata(name);
        Self {
            metadata,
            ..Self::default() // immutable: todo!(),
                              // data: todo!(),
                              // string_data: todo!(),
                              // type_: todo!(),
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
