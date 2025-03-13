use super::*;

pub trait RoleExt: super::ResourceBuilder {
    fn new(name: impl ToString) -> Self;

    fn rule(self, rule: rbacv1::PolicyRule) -> Self {
        self.rules([rule])
    }

    fn rules(self, rules: impl IntoIterator<Item = rbacv1::PolicyRule>) -> Self;
}

impl RoleExt for rbacv1::Role {
    fn new(name: impl ToString) -> Self {
        let metadata = metadata(name);
        Self {
            metadata,
            // rules: todo!(),
            ..default()
        }
    }

    fn rules(self, rules: impl IntoIterator<Item = rbacv1::PolicyRule>) -> Self {
        let rules = Some(rules.into_iter().collect());
        Self { rules, ..self }
    }
}
