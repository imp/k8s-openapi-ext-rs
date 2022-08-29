use super::*;

pub trait LabelSelectorExt {
    fn all_objects() -> Self;
    fn no_objects() -> Self;
    fn new() -> Self;

    fn match_labels(
        self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;
}

impl LabelSelectorExt for metav1::LabelSelector {
    fn new() -> Self {
        default()
    }

    fn all_objects() -> Self {
        Self {
            match_expressions: Some(vec![]),
            match_labels: Some(BTreeMap::new()),
        }
    }

    fn no_objects() -> Self {
        Self {
            match_expressions: None,
            match_labels: None,
        }
    }

    fn match_labels(
        self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        let match_labels = match_labels
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        Self {
            match_labels: Some(match_labels),
            // match_expressions: todo!(),
            ..default()
        }
    }
}
