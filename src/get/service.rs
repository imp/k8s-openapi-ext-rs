use super::*;

pub trait ServiceGetExt {
    fn spec(&self) -> Option<&corev1::ServiceSpec>;
    fn status(&self) -> Option<&corev1::ServiceStatus>;

    fn cluster_ip(&self) -> Option<&str> {
        self.spec()?.cluster_ip.as_deref()
    }

    fn cluster_ips(&self) -> Option<&[String]> {
        self.spec()?.cluster_ips.as_deref()
    }

    fn ports(&self) -> Option<&[corev1::ServicePort]> {
        self.spec()?.ports.as_deref()
    }

    fn port(&self, name: &str) -> Option<&corev1::ServicePort> {
        self.ports()?
            .iter()
            .find(|port| port.name.as_deref() == Some(name))
    }

    fn external_name(&self) -> Option<&str> {
        self.spec()?.external_name.as_deref()
    }

    fn external_ips(&self) -> Option<&[String]> {
        self.spec()?.external_ips.as_deref()
    }

    fn selector(&self) -> Option<&BTreeMap<String, String>> {
        self.spec()?.selector.as_ref()
    }

    fn r#type(&self) -> Option<&str> {
        self.spec()?.type_.as_deref()
    }

    fn load_balancer_ingress(&self) -> Option<&[corev1::LoadBalancerIngress]> {
        self.status()?
            .load_balancer
            .as_ref()
            .and_then(|lb| lb.ingress.as_deref())
    }

    fn is_cluster_ip(&self) -> bool;

    fn is_external_name(&self) -> bool;

    fn is_node_port(&self) -> bool;

    fn is_load_balancer(&self) -> bool;

    fn conditions(&self) -> Option<&[metav1::Condition]> {
        self.status()?.conditions.as_deref()
    }
}

impl ServiceGetExt for corev1::Service {
    fn spec(&self) -> Option<&corev1::ServiceSpec> {
        self.spec.as_ref()
    }

    fn status(&self) -> Option<&corev1::ServiceStatus> {
        self.status.as_ref()
    }

    fn is_cluster_ip(&self) -> bool {
        self.r#type()
            .is_none_or(|r#type| r#type == <Self as ServiceExt>::CLUSTER_IP)
    }

    fn is_external_name(&self) -> bool {
        self.r#type()
            .is_some_and(|r#type| r#type == <Self as ServiceExt>::EXTERNAL_NAME)
    }

    fn is_node_port(&self) -> bool {
        self.r#type()
            .is_some_and(|r#type| r#type == <Self as ServiceExt>::NODE_PORT)
    }

    fn is_load_balancer(&self) -> bool {
        self.r#type()
            .is_some_and(|r#type| r#type == <Self as ServiceExt>::LOAD_BALANCER)
    }
}
