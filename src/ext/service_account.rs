use super::*;

pub trait ServiceAccountExt: super::ResourceBuilder {
    fn new(name: impl ToString) -> Self;

    fn automount_service_account_token(self, yes: bool) -> Self;

    fn image_pull_secret(self, secret: impl ToString) -> Self;

    fn image_pull_secrets(self, secrets: impl IntoIterator<Item = impl ToString>) -> Self;
}

impl ServiceAccountExt for corev1::ServiceAccount {
    fn new(name: impl ToString) -> Self {
        let metadata = metadata(name);
        Self {
            metadata,
            // automount_service_account_token: todo!(),
            // image_pull_secrets: todo!(),
            // secrets: todo!(),
            ..default()
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
        let secret = corev1::LocalObjectReference::new(name);
        let image_pull_secrets = Some(vec![secret]);
        Self {
            image_pull_secrets,
            ..self
        }
    }

    fn image_pull_secrets(self, secrets: impl IntoIterator<Item = impl ToString>) -> Self {
        let secrets = secrets
            .into_iter()
            .map(corev1::LocalObjectReference::new)
            .collect();
        let image_pull_secrets = Some(secrets);
        Self {
            image_pull_secrets,
            ..self
        }
    }
}
