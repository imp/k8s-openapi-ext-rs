use super::*;

/// Extension trait for `corev1::Node`.
/// Fluent builders and mutable accessors
///
pub trait NodeExt: ResourceBuilder {
    /// Creates new `corev1::Node object with given `name`
    ///
    fn new(name: impl ToString) -> Self;
}

impl NodeExt for corev1::Node {
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
