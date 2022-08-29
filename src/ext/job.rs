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
        let metadata = Self::metadata(name);
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

    fn active_deadline_seconds(self, seconds: i64) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.active_deadline_seconds = Some(seconds);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn backoff_limit(self, limit: i32) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.backoff_limit = Some(limit);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn completion_mode<'a>(self, mode: impl Into<Option<&'a str>>) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.completion_mode = mode.into().map(|mode| mode.to_string());
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn completions(self, completions: i32) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.completions = Some(completions);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn manual_selector(self, yes: bool) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.manual_selector = Some(yes);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn parallelism(self, parallelism: i32) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.parallelism = Some(parallelism);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn selector(self, selector: metav1::LabelSelector) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.selector = Some(selector);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn match_labels(
        self,
        match_labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        let selector = spec.selector.unwrap_or_default().match_labels(match_labels);
        spec.selector = Some(selector);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn suspend(self, yes: bool) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.suspend = Some(yes);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn template(self, template: corev1::PodTemplateSpec) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.template = template;
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn pod(self, pod_spec: corev1::PodSpec) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.template.spec = Some(pod_spec);
        Self {
            spec: Some(spec),
            ..self
        }
    }

    fn ttl_seconds_after_finished(self, seconds: i32) -> Self {
        let mut spec = self.spec.unwrap_or_default();
        spec.ttl_seconds_after_finished = Some(seconds);
        Self {
            spec: Some(spec),
            ..self
        }
    }
}
