use super::*;

pub trait TypedObjectReferenceExt {
    /// Construct this `TypedObjectReference` from the given object
    fn from_object<T>(object: &T) -> Self
    where
        T: Resource + openapi::Metadata<Ty = metav1::ObjectMeta>;

    /// Set namespace
    fn namespace(self, namespace: impl ToString) -> Self;
}

impl TypedObjectReferenceExt for corev1::TypedObjectReference {
    fn from_object<T>(object: &T) -> Self
    where
        T: Resource + openapi::Metadata<Ty = metav1::ObjectMeta>,
    {
        let api_group = none_if_empty(T::GROUP);
        let kind = T::KIND.to_string();
        let name = object.metadata().name.clone().unwrap_or_default();
        let namespace = object.metadata().namespace.clone();

        Self {
            api_group,
            kind,
            name,
            namespace,
        }
    }

    fn namespace(self, namespace: impl ToString) -> Self {
        let namespace = Some(namespace.to_string());
        Self { namespace, ..self }
    }
}

fn none_if_empty(text: &str) -> Option<String> {
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use crate::SecretExt;
    #[test]
    fn from_secret() {
        let secret = corev1::Secret::new("NAME");
        let tor = corev1::TypedObjectReference::from_object(&secret);
        assert_eq!(tor.name, "NAME");
        assert_eq!(tor.api_group, None);
        assert_eq!(tor.kind, "Secret");
    }
}
