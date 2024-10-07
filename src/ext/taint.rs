use super::*;

pub trait TaintExt {
    fn no_schedule(key: impl ToString) -> Self;
    fn prefer_no_schedule(key: impl ToString) -> Self;
    fn no_execute(key: impl ToString) -> Self;
    fn value(self, value: impl ToString) -> Self;
}

impl TaintExt for corev1::Taint {
    fn no_schedule(key: impl ToString) -> Self {
        let effect = Effect::NoSchedule.to_string();
        let key = key.to_string();
        Self {
            effect,
            key,
            ..default()
        }
    }

    fn prefer_no_schedule(key: impl ToString) -> Self {
        let effect = Effect::PreferNoSchedule.to_string();
        let key = key.to_string();
        Self {
            effect,
            key,
            ..default()
        }
    }

    fn no_execute(key: impl ToString) -> Self {
        let effect = Effect::NoExecute.to_string();
        let key = key.to_string();
        let time_added = Some(metav1::Time::now());
        Self {
            effect,
            key,
            time_added,
            ..default()
        }
    }

    fn value(self, value: impl ToString) -> Self {
        Self {
            value: Some(value.to_string()),
            ..self
        }
    }
}
