use super::*;

/// Extension trait for `corev1::Node`.
/// Fluent builders and mutable accessors
///
pub trait NodeExt: ResourceBuilder {
    /// Creates new `corev1::Node object with given `name`
    ///
    fn new(name: impl ToString) -> Self;

    /// Sets node taints
    ///
    fn taints(self, taints: impl IntoIterator<Item = corev1::Taint>) -> Self;

    /// Marks node as unschedulable
    ///
    fn unschedulable(self) -> Self;
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

    fn taints(mut self, taints: impl IntoIterator<Item = corev1::Taint>) -> Self {
        let taints = taints.into_iter().collect();
        self.spec_mut().taints = Some(taints);
        self
    }

    fn unschedulable(mut self) -> Self {
        self.spec_mut().unschedulable = Some(true);
        self
    }
}

impl HasSpec for corev1::Node {
    type Spec = corev1::NodeSpec;

    fn spec_mut(&mut self) -> &mut Self::Spec {
        self.spec.get_or_insert_default()
    }
}
