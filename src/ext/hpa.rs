use super::*;

pub trait HorizontalPodAutoscalerExt: super::ResourceBuilder + Sized {
    fn new<T: ScaleTargetRef>(name: impl ToString, scale_target_ref: &T) -> Self;

    fn with_max_replicas<T: ScaleTargetRef>(
        name: impl ToString,
        max: i32,
        scale_target_ref: &T,
    ) -> Self;

    fn max_replicas(self, max_replicas: i32) -> Self;

    fn min_replicas(self, min_replicas: i32) -> Self;
}

impl HorizontalPodAutoscalerExt for autoscalingv2::HorizontalPodAutoscaler {
    fn new<T: ScaleTargetRef>(name: impl ToString, scale_target_ref: &T) -> Self {
        let metadata = metadata(name);
        let scale_target_ref = scale_target_ref.scale_target_ref();
        let spec = autoscalingv2::HorizontalPodAutoscalerSpec {
            scale_target_ref,
            // behavior: todo!(),
            // max_replicas: todo!(),
            // metrics: todo!(),
            // min_replicas: todo!(),
            ..default()
        };

        Self {
            metadata,
            spec: Some(spec),
            // status: todo!(),
            ..default()
        }
    }

    fn with_max_replicas<T: ScaleTargetRef>(
        name: impl ToString,
        max_replicas: i32,
        scale_target_ref: &T,
    ) -> Self {
        let metadata = metadata(name);
        let scale_target_ref = scale_target_ref.scale_target_ref();
        let spec = autoscalingv2::HorizontalPodAutoscalerSpec {
            max_replicas,
            scale_target_ref,
            // behavior: todo!(),
            // metrics: todo!(),
            // min_replicas: todo!(),
            ..default()
        };

        Self {
            metadata,
            spec: Some(spec),
            // status: todo!(),
            ..default()
        }
    }

    fn max_replicas(mut self, max_replicas: i32) -> Self {
        self.spec_mut().max_replicas = max_replicas;
        self
    }

    fn min_replicas(mut self, min_replicas: i32) -> Self {
        self.spec_mut().min_replicas.replace(min_replicas);
        self
    }
}

impl HasSpec for autoscalingv2::HorizontalPodAutoscaler {
    type Spec = autoscalingv2::HorizontalPodAutoscalerSpec;

    fn spec_mut(&mut self) -> &mut Self::Spec {
        self.spec.get_or_insert_with(default)
    }
}

pub trait ScaleTargetRef: openapi::Metadata<Ty = metav1::ObjectMeta> {
    fn scale_target_ref(&self) -> autoscalingv2::CrossVersionObjectReference;
}

impl ScaleTargetRef for appsv1::Deployment {
    fn scale_target_ref(&self) -> autoscalingv2::CrossVersionObjectReference {
        let api_version = openapi::api_version(self).to_string();
        let kind = openapi::kind(self).to_string();
        let name = self.metadata().name.clone().unwrap_or_default();

        autoscalingv2::CrossVersionObjectReference {
            api_version: Some(api_version),
            kind,
            name,
        }
    }
}
