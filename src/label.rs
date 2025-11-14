//! Common Kubernetes label keys

/// The name of the application
pub const APP_NAME: &str = "app.kubernetes.io/name";

/// A unique name identifying the instance of an application
pub const APP_INSTANCE: &str = "app.kubernetes.io/instance";

/// The current version of the application (e.g., a SemVer 1.0, revision hash, etc.)
pub const APP_VERSION: &str = "app.kubernetes.io/version";

/// The component within the architecture
pub const APP_COMPONENT: &str = "app.kubernetes.io/component";

/// The name of a higher level application this one is part of
pub const APP_PART_OF: &str = "app.kubernetes.io/part-of";

/// The tool being used to manage the operation of an application
pub const APP_MANAGED_BY: &str = "app.kubernetes.io/managed-by";

pub const KUBERNETES_IO_ARCH: &str = "kubernetes.io/arch";
pub const KUBERNETES_IO_OS: &str = "kubernetes.io/os";
pub const KUBERNETES_IO_NAME: &str = "kubernetes.io/name";
pub const KUBERNETES_IO_METADATA_NAME: &str = "kubernetes.io/metadata.name";

/// Deprecated
pub const CLUSTER_SERVICE: &str = "kubernetes.io/cluster-service";

pub const DEFAULT_DEPLOYMENT_UNIQUE_LABEL_KEY: &str = "pod-template-hash";
