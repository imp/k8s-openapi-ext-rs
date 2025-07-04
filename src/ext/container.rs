use super::*;

pub trait ContainerExt: Sized {
    fn new(name: impl ToString) -> Self;

    fn command(self, command: impl IntoIterator<Item = impl ToString>) -> Self;

    /// Add environment variables to the container environment list
    ///
    fn env(self, env: impl IntoIterator<Item = impl ToEnvVar>) -> Self;

    /// Add environment variables to the container environment list
    ///
    fn env_from(self, env: impl IntoIterator<Item = impl ToEnvFrom>) -> Self;

    fn image(self, image: impl ToString) -> Self;

    fn image_pull_policy(self, policy: impl ToString) -> Self;

    fn image_pull_policy_always(self) -> Self {
        self.image_pull_policy("Always")
    }

    fn image_pull_policy_never(self) -> Self {
        self.image_pull_policy("Never")
    }

    /// Add `ports` to container ports list
    ///
    fn ports(self, ports: impl IntoIterator<Item = corev1::ContainerPort>) -> Self;

    fn security_context(self, security_context: corev1::SecurityContext) -> Self;

    fn allow_privilege_escalation(mut self, yes: bool) -> Self {
        self.security_context_mut().allow_privilege_escalation = Some(yes);
        self
    }

    fn read_only_root_filesystem(mut self, yes: bool) -> Self {
        self.security_context_mut().read_only_root_filesystem = Some(yes);
        self
    }

    fn run_as_user(mut self, user: i64) -> Self {
        self.security_context_mut().run_as_user = Some(user);
        self
    }

    fn run_as_group(mut self, group: i64) -> Self {
        self.security_context_mut().run_as_group = Some(group);
        self
    }

    fn run_as_non_root(mut self, yes: bool) -> Self {
        self.security_context_mut().run_as_non_root = Some(yes);
        self
    }

    fn privileged(mut self, yes: bool) -> Self {
        self.security_context_mut().privileged = Some(yes);
        self
    }

    /// Add `capabilities` to 'add' list
    ///
    fn add_capabilities(mut self, capabilities: impl IntoIterator<Item = impl ToString>) -> Self {
        let add = capabilities.into_iter().map(|item| item.to_string());
        self.security_context_mut()
            .capabilities
            .get_or_insert_default()
            .add
            .get_or_insert_default()
            .extend(add);
        self
    }

    /// Add `capabilities` to 'drop' list
    ///
    fn drop_capabilities(mut self, capabilities: impl IntoIterator<Item = impl ToString>) -> Self {
        let drop = capabilities.into_iter().map(|item| item.to_string());
        self.security_context_mut()
            .capabilities
            .get_or_insert_default()
            .drop
            .get_or_insert_default()
            .extend(drop);
        self
    }

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

    fn security_context_mut(&mut self) -> &mut corev1::SecurityContext;

    fn working_dir(self, dir: impl ToString) -> Self;
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

    fn command(self, command: impl IntoIterator<Item = impl ToString>) -> Self {
        let command = command.into_iter().map(|item| item.to_string()).collect();
        Self {
            command: Some(command),
            ..self
        }
    }

    fn env(mut self, env: impl IntoIterator<Item = impl ToEnvVar>) -> Self {
        let env = env.into_iter().map(|envvar| ToEnvVar::to_envvar(&envvar));
        self.env.get_or_insert_default().extend(env);
        self
    }

    fn env_from(mut self, env: impl IntoIterator<Item = impl ToEnvFrom>) -> Self {
        let env = env.into_iter().map(ToEnvFrom::to_envfrom);
        self.env_from.get_or_insert_default().extend(env);
        self
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

    fn ports(mut self, ports: impl IntoIterator<Item = corev1::ContainerPort>) -> Self {
        self.ports.get_or_insert_default().extend(ports);
        self
    }

    fn security_context(self, security_context: corev1::SecurityContext) -> Self {
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
            .get_or_insert_default()
            .limits
            .get_or_insert_default()
            .extend(limits);
        self
    }

    fn resource_requests(
        mut self,
        requests: impl IntoIterator<Item = (String, resource::Quantity)>,
    ) -> Self {
        self.resources
            .get_or_insert_default()
            .requests
            .get_or_insert_default()
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

    fn security_context_mut(&mut self) -> &mut corev1::SecurityContext {
        self.security_context.get_or_insert_default()
    }

    fn working_dir(self, dir: impl ToString) -> Self {
        let working_dir = Some(dir.to_string());
        Self {
            working_dir,
            ..self
        }
    }
}
