use super::*;

pub trait OwnerReferenceExt {
    /// Check for `self` being owned by `owner`
    ///
    fn is_owned_by<T>(&self, owner: &T) -> bool
    where
        T: openapi::Metadata<Ty = metav1::ObjectMeta>;

    /// Check for `self` being owned and controlled by `owner`
    ///
    fn is_controlled_by<T>(&self, owner: &T) -> bool
    where
        T: openapi::Metadata<Ty = metav1::ObjectMeta>;
}

impl<K> OwnerReferenceExt for K
where
    K: openapi::Metadata<Ty = metav1::ObjectMeta>,
{
    fn is_owned_by<T>(&self, owner: &T) -> bool
    where
        T: openapi::Metadata<Ty = metav1::ObjectMeta>,
    {
        let name = owner.metadata().name.as_deref().unwrap_or_default();
        let uid = owner.metadata().uid.as_deref().unwrap_or_default();
        self.metadata()
            .owner_references
            .as_deref()
            .unwrap_or_default()
            .iter()
            .any(|owner_ref| {
                owner_ref.api_version == T::API_VERSION
                    && owner_ref.kind == T::KIND
                    && owner_ref.name == name
                    && owner_ref.uid == uid
            })
    }

    fn is_controlled_by<T>(&self, owner: &T) -> bool
    where
        T: openapi::Metadata<Ty = metav1::ObjectMeta>,
    {
        let name = owner.metadata().name.as_deref().unwrap_or_default();
        let uid = owner.metadata().uid.as_deref().unwrap_or_default();
        self.metadata()
            .owner_references
            .as_deref()
            .unwrap_or_default()
            .iter()
            .any(|owner_ref| {
                owner_ref.controller.unwrap_or_default()
                    && owner_ref.api_version == T::API_VERSION
                    && owner_ref.kind == T::KIND
                    && owner_ref.name == name
                    && owner_ref.uid == uid
            })
    }
}
