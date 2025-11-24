use super::*;

pub trait TimeExt {
    fn now() -> Self;

    fn to_system_time(&self) -> std::time::SystemTime;

    fn from_system_time(time: std::time::SystemTime) -> Self;

    #[cfg(feature = "time")]
    fn to_utc_date_time(&self) -> ::time::UtcDateTime;

    #[cfg(feature = "time")]
    fn from_utc_date_time(time: ::time::UtcDateTime) -> Self;

    #[cfg(feature = "jiff")]
    fn from_zoned(time: jiff::Zoned) -> Self;

    #[cfg(feature = "jiff")]
    fn try_to_zoned(&self) -> Result<jiff::Zoned, jiff::Error> {
        let system_time = self.to_system_time();
        jiff::Zoned::try_from(system_time)
    }
}

impl TimeExt for metav1::Time {
    fn now() -> Self {
        Self(openapi::chrono::Utc::now())
    }

    fn to_system_time(&self) -> std::time::SystemTime {
        self.0.into()
    }

    fn from_system_time(time: std::time::SystemTime) -> Self {
        Self(time.into())
    }

    #[cfg(feature = "jiff")]
    fn from_zoned(time: jiff::Zoned) -> Self {
        Self::from_system_time(time.into())
    }

    #[cfg(feature = "time")]
    fn to_utc_date_time(&self) -> ::time::UtcDateTime {
        self.to_system_time().into()
    }

    #[cfg(feature = "time")]
    fn from_utc_date_time(time: ::time::UtcDateTime) -> Self {
        Self::from_system_time(time.into())
    }
}
