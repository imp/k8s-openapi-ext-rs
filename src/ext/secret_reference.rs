use super::*;

pub trait SecretReferenceExt {
    fn new(name: impl ToString) -> Self;
    fn namespace(self, namespace: impl ToString) -> Self;
}

impl SecretReferenceExt for corev1::SecretReference {
    fn new(name: impl ToString) -> Self {
        let name = Some(name.to_string());
        Self { name, ..default() }
    }

    fn namespace(self, namespace: impl ToString) -> Self {
        let namespace = Some(namespace.to_string());
        Self { namespace, ..self }
    }
}
