use super::*;

pub trait StorageClassExt: super::ResourceBuilder {
    fn with_provisioner(name: impl ToString, provisioner: impl ToString) -> Self;

    fn allow_volume_expansion(self, yes: bool) -> Self;
    fn mount_options(self, options: impl IntoIterator<Item = impl ToString>) -> Self;
    fn parameters(
        self,
        parameters: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;
    fn retain(self) -> Self;
    fn delete(self) -> Self;
    fn immediate(self) -> Self;
    fn wait_for_first_consumer(self) -> Self;
}

impl StorageClassExt for storagev1::StorageClass {
    fn with_provisioner(name: impl ToString, provisioner: impl ToString) -> Self {
        let metadata = metadata(name);
        let provisioner = provisioner.to_string();
        Self {
            metadata,
            provisioner,
            // allow_volume_expansion: todo!(),
            // allowed_topologies: todo!(),
            // mount_options: todo!(),
            // parameters: todo!(),
            // reclaim_policy: todo!(),
            // volume_binding_mode: todo!(),
            ..default()
        }
    }

    fn allow_volume_expansion(self, yes: bool) -> Self {
        Self {
            allow_volume_expansion: Some(yes),
            ..self
        }
    }

    fn mount_options(self, options: impl IntoIterator<Item = impl ToString>) -> Self {
        let mount_options = options
            .into_iter()
            .map(|option| option.to_string())
            .collect();

        Self {
            mount_options: Some(mount_options),
            ..self
        }
    }

    fn parameters(
        self,
        parameters: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        let parameters = parameters
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        Self {
            parameters: Some(parameters),
            ..self
        }
    }

    fn retain(self) -> Self {
        Self {
            reclaim_policy: Some(String::from("Retain")),
            ..self
        }
    }

    fn delete(self) -> Self {
        Self {
            reclaim_policy: Some(String::from("Delete")),
            ..self
        }
    }

    fn immediate(self) -> Self {
        Self {
            volume_binding_mode: Some(String::from("Immediate")),
            ..self
        }
    }

    fn wait_for_first_consumer(self) -> Self {
        Self {
            volume_binding_mode: Some(String::from("WaitForFirstConsumer")),
            ..self
        }
    }
}
