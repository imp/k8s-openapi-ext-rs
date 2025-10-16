use super::*;

pub trait PodConditionGetExt {
    // These are built-in conditions of pod. An application may use a custom condition not listed here.

    /// PodScheduled represents status of the scheduling process for this pod
    const POD_SCHEDULED: &str = "PodScheduled";

    /// PodReadyToStartContainers pod sandbox is successfully configured and
    /// the pod is ready to launch containers (beta feature; enabled by default)
    const POD_READY_TO_START_CONTAINERS: &str = "PodReadyToStartContainers";

    /// ContainersReady indicates whether all containers in the pod are ready
    const CONTAINERS_READY: &str = "ContainersReady";

    /// PodInitialized means that all init containers in the pod have started successfully
    const POD_INITIALIZED: &str = "Initialized";

    /// PodReady means the pod is able to service requests and should be added to the
    /// load balancing pools of all matching services
    const POD_READY: &str = "Ready";

    /// DisruptionTarget indicates the pod is about to be terminated due to a
    /// disruption (such as preemption, eviction API or garbage-collection)
    const DISRUPTION_TARGET: &str = "DisruptionTarget";

    /// PodResizePending indicates that the pod has been resized, but kubelet has not
    /// yet allocated the resources. If both PodResizePending and PodResizeInProgress
    /// are set, it means that a new resize was requested in the middle of a previous
    /// pod resize that is still in progress.
    const POD_RESIZE_PENDING: &str = "PodResizePending";

    /// PodResizeInProgress indicates that a resize is in progress, and is present whenever
    /// the Kubelet has allocated resources for the resize, but has not yet actuated all of
    /// the required changes.
    /// If both PodResizePending and PodResizeInProgress are set, it means that a new resize was
    /// requested in the middle of a previous pod resize that is still in progress.
    const POD_RESIZE_IN_PROGRESS: &str = "PodResizeInProgress";

    // These are reasons for a pod's transition to a condition.

    /// PodReasonUnschedulable reason in PodScheduled PodCondition means that the scheduler
    /// can't schedule the pod right now, for example due to insufficient resources in the cluster.
    const POD_REASON_UNSCHEDULABLE: &str = "Unschedulable";

    /// PodReasonSchedulingGated reason in PodScheduled PodCondition means that the scheduler
    /// skips scheduling the pod because one or more scheduling gates are still present.
    const POD_REASON_SCHEDULING_GATED: &str = "SchedulingGated";

    /// PodReasonSchedulerError reason in PodScheduled PodCondition means that some internal error happens
    /// during scheduling, for example due to nodeAffinity parsing errors.
    const POD_REASON_SCHEDULER_ERROR: &str = "SchedulerError";

    /// PodReasonTerminationByKubelet reason in DisruptionTarget pod condition indicates that the termination
    /// is initiated by kubelet
    const POD_REASON_TERMINATION_BY_KUBELET: &str = "TerminationByKubelet";

    /// PodReasonPreemptionByScheduler reason in DisruptionTarget pod condition indicates that the
    /// disruption was initiated by scheduler's preemption.
    const POD_REASON_PREEMPTION_BY_SCHEDULER: &str = "PreemptionByScheduler";

    /// PodReasonDeferred reason in PodResizePending pod condition indicates the proposed resize is feasible in
    /// theory (it fits on this node) but is not possible right now.
    const POD_REASON_DEFERRED: &str = "Deferred";

    /// PodReasonInfeasible reason in PodResizePending pod condition indicates the proposed resize is not
    /// feasible and is rejected; it may not be re-evaluated
    const POD_REASON_INFEASIBLE: &str = "Infeasible";

    /// PodReasonError reason in PodResizeInProgress pod condition indicates that an error occurred while
    /// actuating the resize.
    const POD_REASON_ERROR: &str = "Error";

    fn myself(&self) -> &corev1::PodCondition;

    fn status(&self) -> &str {
        &self.myself().status
    }

    fn r#type(&self) -> &str {
        &self.myself().type_
    }

    fn message(&self) -> Option<&str> {
        self.myself().message.as_deref()
    }

    fn reason(&self) -> Option<&str> {
        self.myself().reason.as_deref()
    }

    fn last_probe_time(&self) -> Option<&metav1::Time> {
        self.myself().last_probe_time.as_ref()
    }

    fn last_transition_time(&self) -> Option<&metav1::Time> {
        self.myself().last_transition_time.as_ref()
    }

    fn is_true(&self) -> bool {
        self.status() == "True"
    }

    fn is_false(&self) -> bool {
        self.status() == "False"
    }

    fn is_unknown(&self) -> bool {
        self.status() == "Unknown"
    }
}

impl PodConditionGetExt for corev1::PodCondition {
    #[inline(always)]
    fn myself(&self) -> &corev1::PodCondition {
        self
    }
}
