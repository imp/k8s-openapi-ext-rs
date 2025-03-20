use super::*;

pub trait VolumeExt: Sized {
    fn secret(name: impl ToString, secret: corev1::SecretVolumeSource) -> Self;
    fn configmap(name: impl ToString, configmap: corev1::ConfigMapVolumeSource) -> Self;
}

impl VolumeExt for corev1::Volume {
    fn secret(name: impl ToString, secret: corev1::SecretVolumeSource) -> Self {
        let name = name.to_string();
        Self {
            name,
            secret: Some(secret),
            // aws_elastic_block_store: todo!(),
            // azure_disk: todo!(),
            // azure_file: todo!(),
            // cephfs: todo!(),
            // cinder: todo!(),
            // config_map: todo!(),
            // csi: todo!(),
            // downward_api: todo!(),
            // empty_dir: todo!(),
            // ephemeral: todo!(),
            // fc: todo!(),
            // flex_volume: todo!(),
            // flocker: todo!(),
            // gce_persistent_disk: todo!(),
            // git_repo: todo!(),
            // glusterfs: todo!(),
            // host_path: todo!(),
            // iscsi: todo!(),
            // nfs: todo!(),
            // persistent_volume_claim: todo!(),
            // photon_persistent_disk: todo!(),
            // portworx_volume: todo!(),
            // projected: todo!(),
            // quobyte: todo!(),
            // rbd: todo!(),
            // scale_io: todo!(),
            // storageos: todo!(),
            // vsphere_volume: todo!(),
            ..default()
        }
    }

    fn configmap(name: impl ToString, configmap: corev1::ConfigMapVolumeSource) -> Self {
        let name = name.to_string();
        Self {
            name,
            config_map: Some(configmap),
            // aws_elastic_block_store: todo!(),
            // azure_disk: todo!(),
            // azure_file: todo!(),
            // cephfs: todo!(),
            // cinder: todo!(),
            // config_map: todo!(),
            // csi: todo!(),
            // downward_api: todo!(),
            // empty_dir: todo!(),
            // ephemeral: todo!(),
            // fc: todo!(),
            // flex_volume: todo!(),
            // flocker: todo!(),
            // gce_persistent_disk: todo!(),
            // git_repo: todo!(),
            // glusterfs: todo!(),
            // host_path: todo!(),
            // image: todo!(),
            // iscsi: todo!(),
            // nfs: todo!(),
            // persistent_volume_claim: todo!(),
            // photon_persistent_disk: todo!(),
            // portworx_volume: todo!(),
            // projected: todo!(),
            // quobyte: todo!(),
            // rbd: todo!(),
            // scale_io: todo!(),
            // secret: todo!(),
            // storageos: todo!(),
            // vsphere_volume: todo!(),
            ..default()
        }
    }
}
