use super::*;

/// Builders for `corev1::SecretEnvSource` objects
pub trait SecretEnvSourceExt: Sized {
    /// Constructs `corev1::SecretEnvSource` object from secret of this `name`
    fn secret_name(name: impl ToString) -> Self;

    /// Specifies whether the Secret must be defined

    fn optional(self, yes: bool) -> Self;

    /// Marks this `Secret` as required (equivalent to calling .optional(false))

    fn required(self) -> Self {
        self.optional(false)
    }
}

impl SecretEnvSourceExt for corev1::SecretEnvSource {
    fn secret_name(name: impl ToString) -> Self {
        let name = Some(name.to_string());
        Self { name, ..default() }
    }

    fn optional(self, yes: bool) -> Self {
        Self {
            optional: Some(yes),
            ..self
        }
    }
}
