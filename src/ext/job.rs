use super::*;

pub trait JobExt: super::ResourceBuilder + Sized {
    fn new(name: impl ToString) -> Self;
    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    fn active_deadline_seconds(self, seconds: i64) -> Self;

    fn backoff_limit(self, limit: i32) -> Self;

    fn completion_mode<'a>(self, mode: impl Into<Option<&'a str>>) -> Self;

    fn non_indexed(self) -> Self {
        self.completion_mode("NonIndexed")
    }

    fn indexed(self) -> Self {
        self.completion_mode("Indexed")
    }

    fn completions(self, completions: i32) -> Self;

    fn manual_selector(self, yes: bool) -> Self;
    fn parallelism(self, parallelism: i32) -> Self;

    fn selector(self, selector: metav1::LabelSelector) -> Self;

    fn match_labels(
        self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    fn suspend(self, yes: bool) -> Self;

    fn template(self, template: corev1::PodTemplateSpec) -> Self;

    fn pod(self, pod: corev1::PodSpec) -> Self;

    fn ttl_seconds_after_finished(self, seconds: i32) -> Self;
}

impl JobExt for batchv1::Job {
    fn new(name: impl ToString) -> Self {
        let metadata = metadata(name);
        Self {
            metadata,
            // spec: todo!(),
            // status: todo!(),
            ..default()
        }
    }

    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        Self::new(name).labels(labels)
    }

    fn active_deadline_seconds(mut self, seconds: i64) -> Self {
        self.spec_mut().active_deadline_seconds.replace(seconds);
        self
    }

    fn backoff_limit(mut self, limit: i32) -> Self {
        self.spec_mut().backoff_limit.replace(limit);
        self
    }

    fn completion_mode<'a>(mut self, mode: impl Into<Option<&'a str>>) -> Self {
        self.spec_mut().completion_mode = mode.into().map(|mode| mode.to_string());
        self
    }

    fn completions(mut self, completions: i32) -> Self {
        self.spec_mut().completions.replace(completions);
        self
    }

    fn manual_selector(mut self, yes: bool) -> Self {
        self.spec_mut().manual_selector.replace(yes);
        self
    }

    fn parallelism(mut self, parallelism: i32) -> Self {
        self.spec_mut().parallelism.replace(parallelism);
        self
    }

    fn selector(mut self, selector: metav1::LabelSelector) -> Self {
        self.spec_mut().selector.replace(selector);
        self
    }

    fn match_labels(
        mut self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        self.spec_mut().selector = Some(metav1::LabelSelector::match_labels(match_labels));
        self
    }

    fn suspend(mut self, yes: bool) -> Self {
        self.spec_mut().suspend.replace(yes);
        self
    }

    fn template(mut self, template: corev1::PodTemplateSpec) -> Self {
        self.spec_mut().template = template;
        self
    }

    fn pod(mut self, pod_spec: corev1::PodSpec) -> Self {
        self.spec_mut().template.spec.replace(pod_spec);
        self
    }

    fn ttl_seconds_after_finished(mut self, seconds: i32) -> Self {
        self.spec_mut().ttl_seconds_after_finished.replace(seconds);
        self
    }
}

impl HasSpec for batchv1::Job {
    type Spec = batchv1::JobSpec;

    fn spec_mut(&mut self) -> &mut Self::Spec {
        self.spec.get_or_insert_default()
    }
}
