use super::*;

/// Builders for `corev1::VolumeMount` objects
pub trait VolumeMountExt: Sized {
    fn new(mount_path: impl ToString, volume: &corev1::Volume) -> Self;

    fn read_only(self) -> Self;
}

impl VolumeMountExt for corev1::VolumeMount {
    fn new(mount_path: impl ToString, volume: &corev1::Volume) -> Self {
        let mount_path = mount_path.to_string();
        let name = volume.name.clone();
        Self {
            mount_path,
            name,
            // mount_propagation: todo!(),
            // read_only: todo!(),
            // sub_path: todo!(),
            // sub_path_expr: todo!(),
            ..Self::default()
        }
    }

    fn read_only(mut self) -> Self {
        self.read_only = Some(true);
        self
    }
}
