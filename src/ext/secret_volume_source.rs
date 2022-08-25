use super::*;

/// Builders for `corev1::SecretVolumeSource` objects
pub trait SecretVolumeSourceExt: Sized {
    fn secret_name(name: impl ToString) -> Self;

    fn optional(self, yes: bool) -> Self;

    fn required(self) -> Self {
        self.optional(false)
    }

    fn default_mode(self, mode: i32) -> Self;

    fn items(self, items: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self;
}

impl SecretVolumeSourceExt for corev1::SecretVolumeSource {
    fn secret_name(name: impl ToString) -> Self {
        let secret_name = Some(name.to_string());
        Self {
            secret_name,
            ..Self::default()
        }
    }

    fn optional(mut self, yes: bool) -> Self {
        self.optional = Some(yes);
        self
    }

    fn default_mode(mut self, mode: i32) -> Self {
        self.default_mode = Some(mode);
        self
    }

    fn items(self, items: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> Self {
        let items = Some(
            items
                .into_iter()
                .map(|(key, path)| corev1::KeyToPath {
                    key: key.to_string(),
                    path: path.to_string(),
                    ..corev1::KeyToPath::default()
                })
                .collect(),
        );
        Self { items, ..self }
    }
}
