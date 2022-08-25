use super::*;

pub trait ContainerExt {
    fn new(name: impl ToString) -> Self;

    fn env(self, env: impl IntoIterator<Item = impl ToEnvVar>) -> Self;

    fn env_from(self, env: impl IntoIterator<Item = impl ToEnvFrom>) -> Self;

    fn image(self, image: impl ToString) -> Self;

    fn image_pull_policy_always(self) -> Self;

    fn image_pull_policy_never(self) -> Self;

    fn liveness_probe(self, probe: corev1::Probe) -> Self;

    fn readiness_probe(self, probe: corev1::Probe) -> Self;

    fn startup_probe(self, probe: corev1::Probe) -> Self;

    fn volume_mounts(self, volume_mounts: impl IntoIterator<Item = corev1::VolumeMount>) -> Self;
}

impl ContainerExt for corev1::Container {
    fn new(name: impl ToString) -> Self {
        let name = name.to_string();
        Self {
            name,
            // args: todo!(),
            // command: todo!(),
            // env: todo!(),
            // env_from: todo!(),
            // image: todo!(),
            // image_pull_policy: todo!(),
            // lifecycle: todo!(),
            // liveness_probe: todo!(),
            // ports: todo!(),
            // readiness_probe: todo!(),
            // resources: todo!(),
            // security_context: todo!(),
            // startup_probe: todo!(),
            // stdin: todo!(),
            // stdin_once: todo!(),
            // termination_message_path: todo!(),
            // termination_message_policy: todo!(),
            // tty: todo!(),
            // volume_devices: todo!(),
            // volume_mounts: todo!(),
            // working_dir: todo!(),
            ..Self::default()
        }
    }

    fn env(self, env: impl IntoIterator<Item = impl ToEnvVar>) -> Self {
        let env = Some(
            env.into_iter()
                .map(|envvar| ToEnvVar::to_envvar(&envvar))
                .collect(),
        );
        Self { env, ..self }
    }

    fn env_from(self, env: impl IntoIterator<Item = impl ToEnvFrom>) -> Self {
        let env_from = Some(env.into_iter().map(ToEnvFrom::to_envfrom).collect());
        Self { env_from, ..self }
    }

    fn image(self, image: impl ToString) -> Self {
        let image = Some(image.to_string());
        Self { image, ..self }
    }

    fn image_pull_policy_always(self) -> Self {
        let image_pull_policy = Some(String::from("Always"));
        Self {
            image_pull_policy,
            ..self
        }
    }

    fn image_pull_policy_never(self) -> Self {
        let image_pull_policy = Some(String::from("Never"));
        Self {
            image_pull_policy,
            ..self
        }
    }

    fn liveness_probe(mut self, probe: corev1::Probe) -> Self {
        self.liveness_probe = Some(probe);
        self
    }

    fn readiness_probe(mut self, probe: corev1::Probe) -> Self {
        self.readiness_probe = Some(probe);
        self
    }

    fn startup_probe(mut self, probe: corev1::Probe) -> Self {
        self.startup_probe = Some(probe);
        self
    }

    fn volume_mounts(
        mut self,
        volume_mounts: impl IntoIterator<Item = corev1::VolumeMount>,
    ) -> Self {
        let volume_mounts = volume_mounts.into_iter().collect();
        self.volume_mounts = Some(volume_mounts);
        self
    }
}
