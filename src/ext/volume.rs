use super::*;

pub trait VolumeExt: Sized {
    fn secret(name: impl ToString, secret: corev1::SecretVolumeSource) -> Self;
}

impl VolumeExt for corev1::Volume {
    fn secret(name: impl ToString, secret: corev1::SecretVolumeSource) -> Self {
        let name = name.to_string();
        Self {
            name,
            secret: Some(secret),
            ..Self::default()
        }
    }
}
