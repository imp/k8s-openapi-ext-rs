use super::*;

pub trait ServicePortGetExt {
    fn myself(&self) -> &corev1::ServicePort;

    fn name(&self) -> Option<&str> {
        self.myself().name.as_deref()
    }

    fn port(&self) -> i32 {
        self.myself().port
    }

    fn target_port(&self) -> Option<&intstr::IntOrString> {
        self.myself().target_port.as_ref()
    }

    fn node_port(&self) -> Option<i32> {
        self.myself().node_port
    }

    fn app_protocol(&self) -> Option<&str> {
        self.myself().app_protocol.as_deref()
    }

    fn protocol(&self) -> Option<&str> {
        self.myself().protocol.as_deref()
    }
}

impl ServicePortGetExt for corev1::ServicePort {
    #[inline(always)]
    fn myself(&self) -> &corev1::ServicePort {
        self
    }
}
