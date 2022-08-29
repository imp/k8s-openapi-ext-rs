use super::*;

pub trait NamespaceExt: super::ResourceBuilder {
    fn new(name: impl ToString) -> Self;
}

impl NamespaceExt for corev1::Namespace {
    fn new(name: impl ToString) -> Self {
        let metadata = metadata(name);
        Self {
            metadata,
            // spec: todo!(),
            // status: todo!(),
            ..default()
        }
    }
}
