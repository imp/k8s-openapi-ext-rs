use super::*;

pub trait TolerationExt {
    fn no_schedule(key: impl ToString) -> TolerationBuilder;
    fn prefer_no_schedule(key: impl ToString) -> TolerationBuilder;
    fn no_execute(key: impl ToString) -> TolerationBuilder;
    fn toleration_seconds(self, toleration_seconds: i64) -> Self;
}

impl TolerationExt for corev1::Toleration {
    fn no_schedule(key: impl ToString) -> TolerationBuilder {
        let effect = Some(Effect::NoSchedule);
        let key = Some(key.to_string());
        TolerationBuilder { effect, key }
    }

    fn prefer_no_schedule(key: impl ToString) -> TolerationBuilder {
        let effect = Some(Effect::PreferNoSchedule);
        let key = Some(key.to_string());
        TolerationBuilder { effect, key }
    }

    fn no_execute(key: impl ToString) -> TolerationBuilder {
        let effect = Some(Effect::NoExecute);
        let key = Some(key.to_string());
        TolerationBuilder { effect, key }
    }

    fn toleration_seconds(self, toleration_seconds: i64) -> Self {
        Self {
            toleration_seconds: Some(toleration_seconds),
            ..self
        }
    }
}

#[derive(Debug)]
#[must_use]
pub struct TolerationBuilder {
    effect: Option<Effect>,
    key: Option<String>,
}

impl TolerationBuilder {
    pub fn equal(self, value: impl ToString) -> corev1::Toleration {
        corev1::Toleration {
            effect: self.effect.map(|effect| effect.to_string()),
            key: self.key,
            operator: Some(Operator::Equal.to_string()),
            toleration_seconds: None,
            value: Some(value.to_string()),
        }
    }

    pub fn exists(self) -> corev1::Toleration {
        corev1::Toleration {
            effect: self.effect.map(|effect| effect.as_str().into()),
            key: self.key,
            operator: Some(Operator::Exists.to_string()),
            toleration_seconds: None,
            value: None,
        }
    }
}

#[derive(Debug)]
enum Operator {
    Equal,
    Exists,
}

impl Operator {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Equal => "Equal",
            Self::Exists => "Exists",
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
