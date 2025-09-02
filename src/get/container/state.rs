use super::*;

pub trait ContainerStateTerminatedGetExt {
    fn reason(&self) -> Option<&str>;

    fn exit_code(&self) -> i32;

    fn signal(&self) -> Option<i32>;

    fn access_reason(&self) -> String {
        if let Some(reason) = self.reason() {
            reason.to_string()
        } else if let Some(signal) = self.signal() {
            format!("Signal:{signal}")
        } else {
            format!("ExitCode:{}", self.exit_code())
        }
    }
}

impl ContainerStateTerminatedGetExt for corev1::ContainerStateTerminated {
    fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    fn exit_code(&self) -> i32 {
        self.exit_code
    }

    fn signal(&self) -> Option<i32> {
        self.signal
    }
}

pub trait ContainerStateWaitingGetExt {
    const POD_INITIALIZING: &str = "PodInitializing";
    const CONTAINER_CREATING: &str = "ContainerCreating";

    fn reason(&self) -> Option<&str>;

    fn is_pod_initializing(&self) -> bool {
        self.reason() == Some(Self::POD_INITIALIZING)
    }

    fn is_container_creating(&self) -> bool {
        self.reason() == Some(Self::CONTAINER_CREATING)
    }
}

impl ContainerStateWaitingGetExt for corev1::ContainerStateWaiting {
    fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}
