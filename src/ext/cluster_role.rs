use super::*;

pub trait ClusterRoleExt: super::ResourceBuilder {
    fn new(name: impl ToString) -> Self;

    fn aggregation_rule(self, selectors: impl IntoIterator<Item = metav1::LabelSelector>) -> Self;

    fn rules(self, rules: impl IntoIterator<Item = rbacv1::PolicyRule>) -> Self;
}

impl ClusterRoleExt for rbacv1::ClusterRole {
    fn new(name: impl ToString) -> Self {
        let metadata = Self::metadata(name);
        Self {
            metadata,
            ..Self::default() // aggregation_rule: todo!(),
                              // rules: todo!(),
        }
    }

    fn aggregation_rule(self, selectors: impl IntoIterator<Item = metav1::LabelSelector>) -> Self {
        let cluster_role_selectors = Some(selectors.into_iter().collect());
        let aggregation_rule = Some(rbacv1::AggregationRule {
            cluster_role_selectors,
        });
        Self {
            aggregation_rule,
            ..self
        }
    }

    fn rules(self, rules: impl IntoIterator<Item = rbacv1::PolicyRule>) -> Self {
        let rules = Some(rules.into_iter().collect());
        Self { rules, ..self }
    }
}
