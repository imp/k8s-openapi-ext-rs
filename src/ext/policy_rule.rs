use super::*;

pub trait PolicyRuleExt: Sized {
    fn new<T: Resource>() -> Self;

    fn api_group(self, group: impl ToString) -> Self;

    fn api_groups(self, groups: impl IntoIterator<Item = impl ToString>) -> Self;

    fn resource(self, resource: impl ToString) -> Self;

    fn resources(self, resources: impl IntoIterator<Item = impl ToString>) -> Self;

    fn with_status(self) -> Self;

    fn verb(self, verb: impl ToString) -> Self;

    fn verbs(self, verbs: impl IntoIterator<Item = impl ToString>) -> Self;

    fn all_resources(self) -> Self {
        self.resource("*")
    }

    fn all_verbs(self) -> Self {
        self.verb("*")
    }
}

impl PolicyRuleExt for rbacv1::PolicyRule {
    fn new<T: Resource>() -> Self {
        default::<Self>()
            .api_group(T::GROUP)
            .resource(T::URL_PATH_SEGMENT)
    }

    fn api_group(self, group: impl ToString) -> Self {
        let api_groups = Some(vec![group.to_string()]);
        Self { api_groups, ..self }
    }

    fn api_groups(self, groups: impl IntoIterator<Item = impl ToString>) -> Self {
        let api_groups = Some(groups.into_iter().map(|group| group.to_string()).collect());
        Self { api_groups, ..self }
    }

    fn resource(self, resource: impl ToString) -> Self {
        let resources = Some(vec![resource.to_string()]);
        Self { resources, ..self }
    }

    fn resources(self, resources: impl IntoIterator<Item = impl ToString>) -> Self {
        let resources = Some(
            resources
                .into_iter()
                .map(|resource| resource.to_string())
                .collect(),
        );
        Self { resources, ..self }
    }

    fn with_status(mut self) -> Self {
        let add_status = |resource: String| {
            if resource.ends_with("/status") {
                vec![resource]
            } else {
                let status = format!("{resource}/status");
                vec![resource, status]
            }
        };

        self.resources = self
            .resources
            .map(|resources| resources.into_iter().flat_map(add_status).collect());
        self
    }

    fn verb(self, verb: impl ToString) -> Self {
        let verbs = vec![verb.to_string()];
        Self { verbs, ..self }
    }

    fn verbs(self, verbs: impl IntoIterator<Item = impl ToString>) -> Self {
        let verbs = verbs.into_iter().map(|verb| verb.to_string()).collect();
        Self { verbs, ..self }
    }
}
