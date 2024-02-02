use super::*;

pub trait MetricExt {
    fn container_resource(source: autoscalingv2::ContainerResourceMetricSource) -> Self;
    fn external(source: autoscalingv2::ExternalMetricSource) -> Self;
    fn object(source: autoscalingv2::ObjectMetricSource) -> Self;
    fn pods(source: autoscalingv2::PodsMetricSource) -> Self;
    fn resource(source: autoscalingv2::ResourceMetricSource) -> Self;
}

impl MetricExt for autoscalingv2::MetricSpec {
    fn container_resource(source: autoscalingv2::ContainerResourceMetricSource) -> Self {
        Self {
            container_resource: Some(source),
            type_: String::from("ContainerResource"),
            ..default()
        }
    }

    fn external(source: autoscalingv2::ExternalMetricSource) -> Self {
        Self {
            external: Some(source),
            type_: String::from("External"),
            ..default()
        }
    }

    fn object(source: autoscalingv2::ObjectMetricSource) -> Self {
        Self {
            object: Some(source),
            type_: String::from("Object"),
            ..default()
        }
    }

    fn pods(source: autoscalingv2::PodsMetricSource) -> Self {
        Self {
            pods: Some(source),
            type_: String::from("Pods"),
            ..default()
        }
    }

    fn resource(source: autoscalingv2::ResourceMetricSource) -> Self {
        Self {
            resource: Some(source),
            type_: String::from("Resource"),
            ..default()
        }
    }
}
