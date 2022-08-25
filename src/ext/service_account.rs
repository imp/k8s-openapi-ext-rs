use super::*;

pub trait ServiceAccountExt: super::ResourceBuilder {
    fn new(name: impl ToString) -> Self;

    fn automount_service_account_token(self, yes: bool) -> Self;

    fn image_pull_secret(self, secret: impl ToString) -> Self;

    fn image_pull_secrets(self, secrets: impl IntoIterator<Item = impl ToString>) -> Self;
}

impl ServiceAccountExt for corev1::ServiceAccount {
    fn new(name: impl ToString) -> Self {
        let metadata = Self::metadata(name);
        Self {
            metadata,
            ..Self::default() // automount_service_account_token: todo!(),
                              // image_pull_secrets: todo!(),
                              // secrets: todo!(),
        }
    }

    fn automount_service_account_token(self, yes: bool) -> Self {
        let automount_service_account_token = Some(yes);
        Self {
            automount_service_account_token,
            ..self
        }
    }

    fn image_pull_secret(self, name: impl ToString) -> Self {
        let name = Some(name.to_string());
        let secret = corev1::LocalObjectReference { name };
        let image_pull_secrets = Some(vec![secret]);
        Self {
            image_pull_secrets,
            ..self
        }
    }

    fn image_pull_secrets(self, secrets: impl IntoIterator<Item = impl ToString>) -> Self {
        let secrets = secrets
            .into_iter()
            .map(|secret| Some(secret.to_string()))
            .map(|name| corev1::LocalObjectReference { name })
            .collect();
        let image_pull_secrets = Some(secrets);
        Self {
            image_pull_secrets,
            ..self
        }
    }
}
