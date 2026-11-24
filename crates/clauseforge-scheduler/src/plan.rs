use clauseforge_kernel::{AmendmentRequest, DomainError};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskKind {
    MapObligations,
    ConstrainChange,
    DraftFallback,
    VerifyControls,
    ApproveAmendment,
    NotifyStakeholders,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MitigationTask {
    pub id: String,
    pub kind: TaskKind,
    pub effort_days: u32,
    pub dependencies: Vec<String>,
    pub required_roles: Vec<String>,
    pub exclusive_resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScheduledTask {
    pub id: String,
    pub kind: TaskKind,
    pub start_day: u32,
    pub finish_day: u32,
    pub lane: usize,
    pub required_roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MitigationPlan {
    pub tasks: Vec<ScheduledTask>,
    pub critical_path_days: u32,
    pub total_work_days: u32,
    pub maximum_parallelism: usize,
    pub cure_window_days: u32,
    pub slack_days: i64,
}

#[derive(Debug, Clone, Default)]
struct Lane {
    available_at: u32,
    resources: BTreeSet<String>,
}

pub fn validate_tasks(tasks: &[MitigationTask]) -> Result<(), DomainError> {
    if tasks.is_empty() {
        return Err(DomainError::Missing("mitigation.tasks"));
    }
    let ids: BTreeSet<_> = tasks.iter().map(|task| task.id.as_str()).collect();
    if ids.len() != tasks.len() {
        return Err(DomainError::Integrity(
            "mitigation task identifiers must be unique".into(),
        ));
    }
    for task in tasks {
        if task.id.trim().is_empty() {
            return Err(DomainError::Missing("mitigation.task.id"));
        }
        if task.effort_days == 0 {
            return Err(DomainError::Range("mitigation.task.duration"));
        }
        if task.required_roles.is_empty() {
            return Err(DomainError::Missing("mitigation.task.roles"));
        }
        for dependency in &task.dependencies {
            if dependency == &task.id {
                return Err(DomainError::Integrity(format!(
                    "mitigation task {} depends on itself",
                    task.id
                )));
            }
            if !ids.contains(dependency.as_str()) {
                return Err(DomainError::Integrity(format!(
                    "mitigation task {} references unknown dependency {}",
                    task.id, dependency
                )));
            }
        }
    }
    Ok(())
}

fn topological_order(tasks: &[MitigationTask]) -> Result<Vec<String>, DomainError> {
    validate_tasks(tasks)?;
    let mut incoming: BTreeMap<String, usize> = tasks
        .iter()
        .map(|task| (task.id.clone(), task.dependencies.len()))
        .collect();
    let mut downstream: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for task in tasks {
        for dependency in &task.dependencies {
            downstream
                .entry(dependency.clone())
                .or_default()
                .push(task.id.clone());
        }
    }
    for dependents in downstream.values_mut() {
        dependents.sort();
    }
    let mut ready: VecDeque<String> = incoming
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(id, _)| id.clone())
        .collect();
    let mut order = Vec::new();
    while let Some(id) = ready.pop_front() {
        order.push(id.clone());
        if let Some(dependents) = downstream.get(&id) {
            for dependent in dependents {
                let count = incoming
                    .get_mut(dependent)
                    .ok_or_else(|| DomainError::Integrity("dependency graph changed".into()))?;
                *count = count.saturating_sub(1);
                if *count == 0 {
                    ready.push_back(dependent.clone());
                }
            }
        }
    }
    if order.len() != tasks.len() {
        return Err(DomainError::Integrity(
            "mitigation task dependency cycle".into(),
        ));
    }
    Ok(order)
}

fn choose_lane(lanes: &[Lane], earliest: u32, exclusive_resource: Option<&str>) -> (usize, u32) {
    lanes
        .iter()
        .enumerate()
        .map(|(index, lane)| {
            let resource_delay = exclusive_resource
                .filter(|resource| lane.resources.contains(*resource))
                .map(|_| lane.available_at)
                .unwrap_or_default();
            (index, earliest.max(lane.available_at).max(resource_delay))
        })
        .min_by_key(|(index, start)| (*start, *index))
        .unwrap_or((0, earliest))
}

pub fn schedule_tasks(
    tasks: &[MitigationTask],
    lane_count: usize,
    cure_window_days: u32,
) -> Result<MitigationPlan, DomainError> {
    validate_tasks(tasks)?;
    if lane_count == 0 {
        return Err(DomainError::Range("mitigation.lanes"));
    }
    if cure_window_days == 0 {
        return Err(DomainError::Range("mitigation.objective"));
    }
    let order = topological_order(tasks)?;
    let by_id: BTreeMap<_, _> = tasks.iter().map(|task| (task.id.as_str(), task)).collect();
    let mut lanes = vec![Lane::default(); lane_count];
    let mut finished = BTreeMap::new();
    let mut scheduled = Vec::with_capacity(tasks.len());

    for id in order {
        let task = by_id
            .get(id.as_str())
            .copied()
            .ok_or_else(|| DomainError::Integrity("ordered task disappeared".into()))?;
        let dependency_finish = task
            .dependencies
            .iter()
            .filter_map(|dependency| finished.get(dependency).copied())
            .max()
            .unwrap_or_default();
        let (lane_index, start_day) = choose_lane(
            &lanes,
            dependency_finish,
            task.exclusive_resource.as_deref(),
        );
        let finish_day = start_day
            .checked_add(task.effort_days)
            .ok_or(DomainError::Range("mitigation.finish"))?;
        let lane = &mut lanes[lane_index];
        lane.available_at = finish_day;
        if let Some(resource) = &task.exclusive_resource {
            lane.resources.insert(resource.clone());
        }
        finished.insert(task.id.clone(), finish_day);
        scheduled.push(ScheduledTask {
            id: task.id.clone(),
            kind: task.kind,
            start_day,
            finish_day,
            lane: lane_index,
            required_roles: task.required_roles.clone(),
        });
    }

    scheduled.sort_by_key(|task| (task.start_day, task.lane, task.id.clone()));
    let critical_path_days = scheduled
        .iter()
        .map(|task| task.finish_day)
        .max()
        .unwrap_or_default();
    let total_work_days = tasks.iter().map(|task| task.effort_days).sum();
    let maximum_parallelism = (0..=critical_path_days)
        .map(|minute| {
            scheduled
                .iter()
                .filter(|task| task.start_day <= minute && minute < task.finish_day)
                .count()
        })
        .max()
        .unwrap_or_default();
    Ok(MitigationPlan {
        tasks: scheduled,
        critical_path_days,
        total_work_days,
        maximum_parallelism,
        cure_window_days,
        slack_days: cure_window_days as i64 - critical_path_days as i64,
    })
}

pub fn tasks_for_request(request: &AmendmentRequest) -> Result<Vec<MitigationTask>, DomainError> {
    request.validate_identity()?;
    let amendment_text = request.amendment.to_ascii_lowercase();
    let control_mapping = (request.affected_controls as u32 / 12).max(3);
    let control_review = (request.cure_days / 8).max(3);
    let approval = (request.cure_days / 3).max(5);
    let data_use_task = if amendment_text.contains("training") || amendment_text.contains("data") {
        MitigationTask {
            id: "verify-data-use-controls".into(),
            kind: TaskKind::VerifyControls,
            effort_days: control_review + 4,
            dependencies: vec!["constrain-amendment".into()],
            required_roles: vec!["data-owner".into(), "auditor".into()],
            exclusive_resource: Some("evidence-store".into()),
        }
    } else {
        MitigationTask {
            id: "verify-obligation-state".into(),
            kind: TaskKind::VerifyControls,
            effort_days: control_review,
            dependencies: vec!["constrain-amendment".into()],
            required_roles: vec!["contract-owner".into()],
            exclusive_resource: Some("amendment-control".into()),
        }
    };
    let review_id = data_use_task.id.clone();
    Ok(vec![
        MitigationTask {
            id: "map-obligations".into(),
            kind: TaskKind::MapObligations,
            effort_days: 3,
            dependencies: vec![],
            required_roles: vec!["risk-analyst".into()],
            exclusive_resource: None,
        },
        MitigationTask {
            id: "notify-control-owners".into(),
            kind: TaskKind::NotifyStakeholders,
            effort_days: 2,
            dependencies: vec!["map-obligations".into()],
            required_roles: vec!["procurement-counsel".into()],
            exclusive_resource: Some("reviewer-channel".into()),
        },
        MitigationTask {
            id: "constrain-amendment".into(),
            kind: TaskKind::ConstrainChange,
            effort_days: control_mapping,
            dependencies: vec!["map-obligations".into()],
            required_roles: vec!["contract-owner".into(), "policy-owner".into()],
            exclusive_resource: Some("amendment-control".into()),
        },
        MitigationTask {
            id: "draft-fallback-terms".into(),
            kind: TaskKind::DraftFallback,
            effort_days: (request.cure_days / 5).max(4),
            dependencies: vec!["constrain-amendment".into()],
            required_roles: vec!["ai-governance-lead".into()],
            exclusive_resource: Some("amendment-control".into()),
        },
        data_use_task,
        MitigationTask {
            id: "approval-primary".into(),
            kind: TaskKind::ApproveAmendment,
            effort_days: approval,
            dependencies: vec!["draft-fallback-terms".into(), review_id],
            required_roles: vec!["contract-owner".into(), "risk-analyst".into()],
            exclusive_resource: Some("amendment-control".into()),
        },
        MitigationTask {
            id: "seal-decision-audit".into(),
            kind: TaskKind::VerifyControls,
            effort_days: control_review,
            dependencies: vec!["approval-primary".into(), "notify-control-owners".into()],
            required_roles: vec!["auditor".into(), "procurement-counsel".into()],
            exclusive_resource: Some("evidence-store".into()),
        },
    ])
}

pub fn plan_request(request: &AmendmentRequest) -> Result<MitigationPlan, DomainError> {
    schedule_tasks(&tasks_for_request(request)?, 3, request.cure_days)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(amendment: &str, cure_days: u32) -> AmendmentRequest {
        AmendmentRequest {
            supplier: "billing-edge".into(),
            amendment: amendment.into(),
            affected_controls: 58,
            cure_days,
            reviewer: "procurement-counsel".into(),
        }
    }

    #[test]
    fn plan_honors_dependencies_and_objective() {
        let plan = plan_request(&request("database failover", 60)).unwrap();
        let finish: BTreeMap<_, _> = plan
            .tasks
            .iter()
            .map(|task| (task.id.as_str(), task.finish_day))
            .collect();
        let approval = plan
            .tasks
            .iter()
            .find(|task| task.id == "approval-primary")
            .unwrap();
        assert!(approval.start_day >= finish["draft-fallback-terms"]);
        assert!(approval.start_day >= finish["verify-data-use-controls"]);
        assert!(plan.slack_days >= 0);
    }

    #[test]
    fn scheduler_uses_parallel_lanes() {
        let plan = plan_request(&request("queue partition", 50)).unwrap();
        assert!(plan.maximum_parallelism >= 2);
        assert!(plan.critical_path_days < plan.total_work_days);
    }

    #[test]
    fn data_scenario_requires_auditor_and_data_owner() {
        let tasks = tasks_for_request(&request("database corruption", 60)).unwrap();
        let verify = tasks
            .iter()
            .find(|task| task.id == "verify-data-use-controls")
            .unwrap();
        assert!(verify.required_roles.contains(&"auditor".to_string()));
        assert!(verify.required_roles.contains(&"data-owner".to_string()));
    }

    #[test]
    fn rejects_unknown_dependency() {
        let mut tasks = tasks_for_request(&request("cache loss", 60)).unwrap();
        tasks[0].dependencies.push("missing".into());
        assert!(
            matches!(validate_tasks(&tasks), Err(DomainError::Integrity(message)) if message.contains("unknown"))
        );
    }

    #[test]
    fn rejects_dependency_cycle() {
        let mut tasks = tasks_for_request(&request("cache loss", 60)).unwrap();
        tasks[0].dependencies.push("seal-decision-audit".into());
        assert!(
            matches!(schedule_tasks(&tasks, 2, 60), Err(DomainError::Integrity(message)) if message.contains("cycle"))
        );
    }

    #[test]
    fn shorter_objective_reports_negative_slack() {
        let plan = plan_request(&request("regional queue loss", 12)).unwrap();
        assert!(plan.slack_days < 0);
    }
}
