use super::*;

pub trait RoleRefExt: Sized {
    fn new<T: openapi::Metadata<Ty = metav1::ObjectMeta>>(k: &T) -> Self;
}

impl RoleRefExt for rbacv1::RoleRef {
    fn new<T: openapi::Metadata<Ty = metav1::ObjectMeta>>(k: &T) -> Self {
        let name = k.metadata().name.clone().unwrap_or_default();
        let api_group = T::GROUP.to_string();
        let kind = T::KIND.to_string();
        Self {
            name,
            api_group,
            kind,
        }
    }
}
