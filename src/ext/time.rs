use super::*;

pub trait TimeExt {
    fn now() -> Self;
}

impl TimeExt for metav1::Time {
    fn now() -> Self {
        Self(openapi::chrono::Utc::now())
    }
}
