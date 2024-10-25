use super::*;

pub trait ConfigMapGetExt {
    fn myself(&self) -> &corev1::ConfigMap;

    fn data(&self) -> Option<&BTreeMap<String, String>> {
        self.myself().data.as_ref()
    }

    fn binary_data(&self) -> Option<&BTreeMap<String, ByteString>> {
        self.myself().binary_data.as_ref()
    }

    fn immutable(&self) -> Option<bool> {
        self.myself().immutable
    }
}

impl ConfigMapGetExt for corev1::ConfigMap {
    #[inline(always)]
    fn myself(&self) -> &corev1::ConfigMap {
        self
    }
}
