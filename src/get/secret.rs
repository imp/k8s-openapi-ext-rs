use super::*;

pub trait SecretGetExt {
    fn myself(&self) -> &corev1::Secret;

    fn r#type(&self) -> Option<&str> {
        self.myself().type_.as_deref()
    }

    fn data(&self) -> Option<&BTreeMap<String, ByteString>> {
        self.myself().data.as_ref()
    }

    fn string_data(&self) -> Option<&BTreeMap<String, String>> {
        self.myself().string_data.as_ref()
    }

    fn immutable(&self) -> Option<bool> {
        self.myself().immutable
    }

    fn item(&self, key: &str) -> Option<&ByteString> {
        self.data()?.get(key)
    }
}

impl SecretGetExt for corev1::Secret {
    #[inline(always)]
    fn myself(&self) -> &corev1::Secret {
        self
    }
}
