use super::*;

pub trait CronJobExt: super::ResourceBuilder + Sized {
    fn new(name: impl ToString) -> Self;
    fn with_labels(
        name: impl ToString,
        labels: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    ) -> Self;

    fn starting_deadline_seconds(self, seconds: i64) -> Self;

    fn schedule(self, schedule: impl ToString) -> Self;

    fn suspend(self, yes: bool) -> Self;

    fn template(self, template: batchv1::JobTemplateSpec) -> Self;
}

impl CronJobExt for batchv1::CronJob {
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

    fn starting_deadline_seconds(mut self, seconds: i64) -> Self {
        self.spec_mut().starting_deadline_seconds.replace(seconds);
        self
    }

    fn schedule(mut self, schedule: impl ToString) -> Self {
        self.spec_mut().schedule = schedule.to_string();
        self
    }

    fn suspend(mut self, yes: bool) -> Self {
        self.spec_mut().suspend.replace(yes);
        self
    }

    fn template(mut self, template: batchv1::JobTemplateSpec) -> Self {
        self.spec_mut().job_template = template;
        self
    }
}

impl HasSpec for batchv1::CronJob {
    type Spec = batchv1::CronJobSpec;

    fn spec_mut(&mut self) -> &mut Self::Spec {
        self.spec.get_or_insert_default()
    }
}
