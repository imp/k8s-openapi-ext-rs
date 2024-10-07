use super::*;

#[derive(Debug)]
pub(super) enum Effect {
    NoSchedule,
    PreferNoSchedule,
    NoExecute,
}

impl Effect {
    pub(super) fn as_str(&self) -> &'static str {
        match self {
            Self::NoSchedule => "NoSchedule",
            Self::PreferNoSchedule => "PreferNoSchedule",
            Self::NoExecute => "NoExecute",
        }
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
