use super::*;

/// Extension trait for `corev1::ConfigMap`.
/// Fluent builders and mutable accessors
///
pub trait ConfigMapExt: ResourceBuilder {
    /// Creates new `corev1::ConfigMap object with given `name`
    ///
    fn new(name: impl ToString) -> Self;

    /// Initializes `immutable` field
    ///
    fn immutable(self, yes: bool) -> Self;

    /// Initializes `binary_data` field
    ///
    fn binary_data(self, data: impl IntoIterator<Item = (impl ToString, ByteString)>) -> Self;

    /// Initializes `data` field
    ///
    fn data(self, data: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self;

    /// Mutable access to `data`.
    /// Initializes `data` with empty `BTreeMap` if absent
    ///
    fn data_mut(&mut self) -> &mut BTreeMap<String, String>;

    /// Mutable access to `binary_data`.
    /// Initializes `binary_data` with empty `BTreeMap` if absent
    ///
    fn binary_data_mut(&mut self) -> &mut BTreeMap<String, ByteString>;
}

impl ConfigMapExt for corev1::ConfigMap {
    fn new(name: impl ToString) -> Self {
        let metadata = Self::metadata(name);
        Self {
            metadata,
            // binary_data: todo!(),
            // data: todo!(),
            // immutable: todo!(),
            ..Self::default()
        }
    }

    fn immutable(self, yes: bool) -> Self {
        let immutable = Some(yes);
        Self { immutable, ..self }
    }

    fn binary_data(self, data: impl IntoIterator<Item = (impl ToString, ByteString)>) -> Self {
        let data = data
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect();
        Self {
            binary_data: Some(data),
            ..self
        }
    }

    fn data(self, data: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self {
        let data = data
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        Self {
            data: Some(data),
            ..self
        }
    }

    fn data_mut(&mut self) -> &mut BTreeMap<String, String> {
        self.data.get_or_insert_with(BTreeMap::new)
    }

    fn binary_data_mut(&mut self) -> &mut BTreeMap<String, ByteString> {
        self.binary_data.get_or_insert_with(BTreeMap::new)
    }
}
