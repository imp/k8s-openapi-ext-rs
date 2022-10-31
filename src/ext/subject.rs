use openapi::Metadata;

use super::*;

pub trait SubjectExt: Sized {
    fn with_kind(name: impl ToString, kind: impl ToString) -> Self;
    fn user(name: impl ToString) -> Self {
        Self::with_kind(name, "User")
    }
    fn group(name: impl ToString) -> Self {
        Self::with_kind(name, "Group")
    }
    fn service_account(account: &corev1::ServiceAccount) -> Self {
        let name = account.metadata().name.clone().unwrap_or_default();
        Self::with_kind(name, corev1::ServiceAccount::KIND)
    }

    fn namespace(self, namespace: impl ToString) -> Self;
}

impl SubjectExt for rbacv1::Subject {
    fn with_kind(name: impl ToString, kind: impl ToString) -> Self {
        let kind = kind.to_string();
        let name = name.to_string();
        Self {
            kind,
            name,
            // api_group: todo!(),
            // namespace: todo!(),
            ..default()
        }
    }

    fn namespace(self, namespace: impl ToString) -> Self {
        let namespace = Some(namespace.to_string());
        Self { namespace, ..self }
    }
}
