use super::*;

pub trait RoleExt: super::ResourceBuilder {
    fn new(name: impl ToString) -> Self;

    fn rules(self, rules: impl IntoIterator<Item = rbacv1::PolicyRule>) -> Self;
}

impl RoleExt for rbacv1::Role {
    fn new(name: impl ToString) -> Self {
        let metadata = Self::metadata(name);
        Self {
            metadata,
            // rules: todo!(),
            ..Self::default()
        }
    }

    fn rules(self, rules: impl IntoIterator<Item = rbacv1::PolicyRule>) -> Self {
        let rules = Some(rules.into_iter().collect());
        Self { rules, ..self }
    }
}
