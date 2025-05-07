use super::*;

pub trait SelfSubjectReviewGetExt {
    fn status(&self) -> Option<&authenticationv1::SelfSubjectReviewStatus>;

    fn userinfo(&self) -> Option<&authenticationv1::UserInfo> {
        self.status()?.user_info.as_ref()
    }

    fn username(&self) -> Option<&str> {
        self.userinfo()?.username.as_deref()
    }

    fn uid(&self) -> Option<&str> {
        self.userinfo()?.uid.as_deref()
    }

    fn groups(&self) -> Option<&[String]> {
        self.userinfo()?.groups.as_deref()
    }

    fn extra(&self) -> Option<&BTreeMap<String, Vec<String>>> {
        self.userinfo()?.extra.as_ref()
    }
}

impl SelfSubjectReviewGetExt for authenticationv1::SelfSubjectReview {
    fn status(&self) -> Option<&authenticationv1::SelfSubjectReviewStatus> {
        self.status.as_ref()
    }
}
