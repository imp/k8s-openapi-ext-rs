use super::*;

pub trait ContainerExt: Sized {
    fn new(name: impl ToString) -> Self;

    fn env(self, env: impl IntoIterator<Item = impl ToEnvVar>) -> Self;

    fn env_from(self, env: impl IntoIterator<Item = impl ToEnvFrom>) -> Self;

    fn image(self, image: impl ToString) -> Self;

    fn image_pull_policy(self, policy: impl ToString) -> Self;

    fn image_pull_policy_always(self) -> Self {
        self.image_pull_policy("Always")
    }

    fn image_pull_policy_never(self) -> Self {
        self.image_pull_policy("Never")
    }

    fn security_context(self, security_context: corev1::SecurityContext) -> Self;
    fn allow_privilege_escalation(self, yes: bool) -> Self;

    fn privileged(self, yes: bool) -> Self;

    fn run_as_user(self, user: i64) -> Self;

    fn run_as_group(self, group: i64) -> Self;

    fn run_as_non_root(self, yes: bool) -> Self;

    fn liveness_probe(self, probe: corev1::Probe) -> Self;

    fn readiness_probe(self, probe: corev1::Probe) -> Self;

    fn resource_limits(
        self,
        limits: impl IntoIterator<Item = (String, resource::Quantity)>,
    ) -> Self;

    fn resource_requests(
        self,
        requests: impl IntoIterator<Item = (String, resource::Quantity)>,
    ) -> Self;

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
            ..default()
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

    fn image_pull_policy(self, policy: impl ToString) -> Self {
        let image_pull_policy = Some(policy.to_string());
        Self {
            image_pull_policy,
            ..self
        }
    }

    fn security_context(self, security_context: corev1::SecurityContext) -> Self {
        Self {
            security_context: Some(security_context),
            ..self
        }
    }

    fn allow_privilege_escalation(self, yes: bool) -> Self {
        let mut security_context = self.security_context.unwrap_or_default();
        security_context.allow_privilege_escalation = Some(yes);
        Self {
            security_context: Some(security_context),
            ..self
        }
    }

    fn run_as_user(self, user: i64) -> Self {
        let mut security_context = self.security_context.unwrap_or_default();
        security_context.run_as_user = Some(user);
        Self {
            security_context: Some(security_context),
            ..self
        }
    }

    fn run_as_group(self, group: i64) -> Self {
        let mut security_context = self.security_context.unwrap_or_default();
        security_context.run_as_group = Some(group);
        Self {
            security_context: Some(security_context),
            ..self
        }
    }

    fn run_as_non_root(self, yes: bool) -> Self {
        let mut security_context = self.security_context.unwrap_or_default();
        security_context.run_as_non_root = Some(yes);
        Self {
            security_context: Some(security_context),
            ..self
        }
    }

    fn privileged(self, yes: bool) -> Self {
        let mut security_context = self.security_context.unwrap_or_default();
        security_context.privileged = Some(yes);
        Self {
            security_context: Some(security_context),
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

    fn resource_limits(
        mut self,
        limits: impl IntoIterator<Item = (String, resource::Quantity)>,
    ) -> Self {
        self.resources
            .get_or_insert_with(default)
            .limits
            .get_or_insert_with(default)
            .extend(limits);
        self
    }

    fn resource_requests(
        mut self,
        requests: impl IntoIterator<Item = (String, resource::Quantity)>,
    ) -> Self {
        self.resources
            .get_or_insert_with(default)
            .requests
            .get_or_insert_with(default)
            .extend(requests);

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
