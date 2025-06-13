use super::*;

pub trait SecurityContextExt {
    fn new() -> Self;

    fn allow_privilege_escalation(self, yes: bool) -> Self;
    fn read_only_root_filesystem(self, yes: bool) -> Self;
    fn run_as_group(self, group: i64) -> Self;
    fn run_as_non_root(self, yes: bool) -> Self;
    fn run_as_user(self, user: i64) -> Self;
    fn privileged(self, yes: bool) -> Self;
    fn add_capabilities(self, capabilities: impl IntoIterator<Item = impl ToString>) -> Self;
    fn drop_capabilities(self, capabilities: impl IntoIterator<Item = impl ToString>) -> Self;
}

impl SecurityContextExt for corev1::SecurityContext {
    fn new() -> Self {
        Self {
            // allow_privilege_escalation: todo!(),
            // app_armor_profile: todo!(),
            // capabilities: todo!(),
            // privileged: todo!(),
            // proc_mount: todo!(),
            // read_only_root_filesystem: todo!(),
            // run_as_group: todo!(),
            // run_as_non_root: todo!(),
            // run_as_user: todo!(),
            // se_linux_options: todo!(),
            // seccomp_profile: todo!(),
            // windows_options: todo!(),
            ..default()
        }
    }

    fn allow_privilege_escalation(self, yes: bool) -> Self {
        Self {
            allow_privilege_escalation: Some(yes),
            ..self
        }
    }

    fn read_only_root_filesystem(self, yes: bool) -> Self {
        Self {
            read_only_root_filesystem: Some(yes),
            ..self
        }
    }

    fn run_as_group(self, group: i64) -> Self {
        Self {
            run_as_group: Some(group),
            ..self
        }
    }

    fn run_as_non_root(self, yes: bool) -> Self {
        Self {
            run_as_non_root: Some(yes),
            ..self
        }
    }

    fn run_as_user(self, user: i64) -> Self {
        Self {
            run_as_user: Some(user),
            ..self
        }
    }

    fn privileged(self, privileged: bool) -> Self {
        Self {
            privileged: Some(privileged),
            ..self
        }
    }

    /// Set capabilities 'add' list
    ///
    fn add_capabilities(mut self, capabilities: impl IntoIterator<Item = impl ToString>) -> Self {
        let add = capabilities
            .into_iter()
            .map(|item| item.to_string())
            .collect();
        self.capabilities.get_or_insert_default().add = Some(add);
        self
    }

    /// Set capabilities 'drop' list
    ///
    fn drop_capabilities(mut self, capabilities: impl IntoIterator<Item = impl ToString>) -> Self {
        let drop = capabilities
            .into_iter()
            .map(|item| item.to_string())
            .collect();
        self.capabilities.get_or_insert_default().drop = Some(drop);
        self
    }
}
