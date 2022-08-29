use super::*;

pub trait ClusterRoleBindingExt: super::ResourceBuilder {
    fn new(name: impl ToString, cluster_role: &rbacv1::ClusterRole) -> Self;

    fn subjects(self, subjects: impl IntoIterator<Item = rbacv1::Subject>) -> Self;
}

impl ClusterRoleBindingExt for rbacv1::ClusterRoleBinding {
    fn new(name: impl ToString, cluster_role: &rbacv1::ClusterRole) -> Self {
        let metadata = metadata(name);
        let role_ref = rbacv1::RoleRef::new(cluster_role);
        Self {
            metadata,
            role_ref,
            // subjects: todo!(),
            ..default()
        }
    }

    fn subjects(self, subjects: impl IntoIterator<Item = rbacv1::Subject>) -> Self {
        let subjects = Some(subjects.into_iter().collect());
        Self { subjects, ..self }
    }
}
