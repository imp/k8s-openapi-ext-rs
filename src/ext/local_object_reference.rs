use super::*;

pub trait LocalObjectReferenceExt {
    fn new(name: impl ToString) -> Self;
}

impl LocalObjectReferenceExt for corev1::LocalObjectReference {
    fn new(name: impl ToString) -> Self {
        let name = name.to_string();
        Self { name: Some(name) }
    }
}
