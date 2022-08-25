use super::*;

pub trait RoleBindingExt: super::ResourceBuilder {
    fn new<T: IsRole>(name: impl ToString, role: &T) -> Self;

    fn subjects(self, subjects: impl IntoIterator<Item = rbacv1::Subject>) -> Self;
}

impl RoleBindingExt for rbacv1::RoleBinding {
    fn new<T: IsRole>(name: impl ToString, role: &T) -> Self
    where
        T: IsRole,
    {
        let metadata = Self::metadata(name);
        let role_ref = rbacv1::RoleRef {
            api_group: T::GROUP.to_string(),
            kind: T::KIND.to_string(),
            name: role.metadata().name.clone().unwrap_or_default(),
        };
        Self {
            metadata,
            role_ref,
            // subjects: todo!(),
            ..Self::default()
        }
    }

    fn subjects(self, subjects: impl IntoIterator<Item = rbacv1::Subject>) -> Self {
        let subjects = Some(subjects.into_iter().collect());
        Self { subjects, ..self }
    }
}

// pub trait IsRole: Resource {}
pub trait IsRole: openapi::Metadata<Ty = metav1::ObjectMeta> {}

impl IsRole for rbacv1::Role {}
impl IsRole for rbacv1::ClusterRole {}
