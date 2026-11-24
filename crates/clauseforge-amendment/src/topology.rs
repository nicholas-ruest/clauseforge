use clauseforge_kernel::{AmendmentRequest, DomainError};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContractNodeKind {
    SupplierBoundary,
    ContractSupplier,
    DataUseObligation,
    NoticeObligation,
    PolicyControl,
    Subprocessor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContractNode {
    pub id: String,
    pub kind: ContractNodeKind,
    pub criticality: u8,
    pub cure_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImpactEdge {
    pub from: String,
    pub to: String,
    pub propagation: f64,
    pub required: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ObligationGraph {
    pub nodes: Vec<ContractNode>,
    pub edges: Vec<ImpactEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Impact {
    pub node_id: String,
    pub probability: f64,
    pub depth: usize,
    pub path: Vec<String>,
    pub weighted_criticality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AmendmentImpactAnalysis {
    pub origin: String,
    pub impacts: Vec<Impact>,
    pub maximum_weighted_impact: f64,
    pub affected_nodes: usize,
    pub longest_path: usize,
    pub remediation_burden_days: u32,
}

impl ObligationGraph {
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.nodes.is_empty() {
            return Err(DomainError::Missing("graph.nodes"));
        }
        let mut ids = BTreeSet::new();
        for node in &self.nodes {
            if node.id.trim().is_empty() {
                return Err(DomainError::Missing("node.id"));
            }
            if !ids.insert(node.id.as_str()) {
                return Err(DomainError::Integrity(format!(
                    "duplicate graph node {}",
                    node.id
                )));
            }
            if !(1..=100).contains(&node.criticality) {
                return Err(DomainError::Range("node.criticality"));
            }
            if node.cure_days == 0 {
                return Err(DomainError::Range("node.cure_days"));
            }
        }
        for edge in &self.edges {
            if !ids.contains(edge.from.as_str()) || !ids.contains(edge.to.as_str()) {
                return Err(DomainError::Integrity(format!(
                    "edge references unknown node {} -> {}",
                    edge.from, edge.to
                )));
            }
            if edge.from == edge.to {
                return Err(DomainError::Integrity(format!(
                    "self edge is not permitted for {}",
                    edge.from
                )));
            }
            if !edge.propagation.is_finite() || !(0.0..=1.0).contains(&edge.propagation) {
                return Err(DomainError::Range("edge.propagation"));
            }
        }
        Ok(())
    }

    pub fn node(&self, id: &str) -> Option<&ContractNode> {
        self.nodes.iter().find(|node| node.id == id)
    }

    pub fn downstream(&self, id: &str) -> Vec<&ImpactEdge> {
        let mut edges: Vec<_> = self.edges.iter().filter(|edge| edge.from == id).collect();
        edges.sort_by(|left, right| {
            right
                .propagation
                .total_cmp(&left.propagation)
                .then_with(|| left.to.cmp(&right.to))
        });
        edges
    }

    pub fn propagate(
        &self,
        origin: &str,
        maximum_depth: usize,
        probability_floor: f64,
    ) -> Result<AmendmentImpactAnalysis, DomainError> {
        self.validate()?;
        if self.node(origin).is_none() {
            return Err(DomainError::Missing("graph.origin"));
        }
        if maximum_depth == 0 {
            return Err(DomainError::Range("graph.maximum_depth"));
        }
        if !probability_floor.is_finite() || !(0.0..=1.0).contains(&probability_floor) {
            return Err(DomainError::Range("graph.probability_floor"));
        }
        let mut best_probability = BTreeMap::new();
        let mut best_path: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut queue = VecDeque::new();
        best_probability.insert(origin.to_string(), 1.0_f64);
        best_path.insert(origin.to_string(), vec![origin.to_string()]);
        queue.push_back((origin.to_string(), 1.0_f64, 0_usize));
        while let Some((current, probability, depth)) = queue.pop_front() {
            if depth >= maximum_depth {
                continue;
            }
            for edge in self.downstream(&current) {
                let required_multiplier = if edge.required { 1.0 } else { 0.72 };
                let candidate = probability * edge.propagation * required_multiplier;
                if candidate < probability_floor {
                    continue;
                }
                let previous = best_probability.get(&edge.to).copied().unwrap_or_default();
                if candidate <= previous {
                    continue;
                }
                let mut path = best_path
                    .get(&current)
                    .cloned()
                    .unwrap_or_else(|| vec![current.clone()]);
                if path.iter().any(|node| node == &edge.to) {
                    continue;
                }
                path.push(edge.to.clone());
                best_probability.insert(edge.to.clone(), candidate);
                best_path.insert(edge.to.clone(), path);
                queue.push_back((edge.to.clone(), candidate, depth + 1));
            }
        }
        let mut impacts = Vec::new();
        for (node_id, probability) in best_probability {
            let node = self
                .node(&node_id)
                .ok_or_else(|| DomainError::Integrity("impact node disappeared".into()))?;
            let path = best_path.remove(&node_id).unwrap_or_default();
            let depth = path.len().saturating_sub(1);
            impacts.push(Impact {
                node_id,
                probability,
                depth,
                path,
                weighted_criticality: probability * node.criticality as f64,
            });
        }
        impacts.sort_by(|left, right| {
            right
                .weighted_criticality
                .total_cmp(&left.weighted_criticality)
                .then_with(|| left.node_id.cmp(&right.node_id))
        });
        let maximum_weighted_impact = impacts
            .iter()
            .map(|impact| impact.weighted_criticality)
            .fold(0.0_f64, f64::max);
        let longest_path = impacts.iter().map(|impact| impact.depth).max().unwrap_or(0);
        let remediation_burden_days = impacts
            .iter()
            .filter_map(|impact| {
                self.node(&impact.node_id)
                    .map(|node| (node.cure_days as f64 * impact.probability).ceil() as u32)
            })
            .sum();
        Ok(AmendmentImpactAnalysis {
            origin: origin.to_string(),
            affected_nodes: impacts.len(),
            impacts,
            maximum_weighted_impact,
            longest_path,
            remediation_burden_days,
        })
    }

    pub fn cut_candidates(&self, origin: &str) -> Result<Vec<(String, f64)>, DomainError> {
        let baseline = self.propagate(origin, self.nodes.len().max(1), 0.01)?;
        let baseline_total: f64 = baseline
            .impacts
            .iter()
            .map(|impact| impact.weighted_criticality)
            .sum();
        let mut candidates = Vec::new();
        for node in &self.nodes {
            if node.id == origin {
                continue;
            }
            let mut reduced = self.clone();
            reduced
                .edges
                .retain(|edge| edge.from != node.id && edge.to != node.id);
            let reduced_analysis = reduced.propagate(origin, reduced.nodes.len().max(1), 0.01)?;
            let reduced_total: f64 = reduced_analysis
                .impacts
                .iter()
                .map(|impact| impact.weighted_criticality)
                .sum();
            candidates.push((node.id.clone(), (baseline_total - reduced_total).max(0.0)));
        }
        candidates.sort_by(|left, right| {
            right
                .1
                .total_cmp(&left.1)
                .then_with(|| left.0.cmp(&right.0))
        });
        Ok(candidates)
    }
}

pub fn obligation_graph_for_request(
    request: &AmendmentRequest,
) -> Result<ObligationGraph, DomainError> {
    request.validate_identity()?;
    let amendment_text = request.amendment.to_ascii_lowercase();
    let supplier_id = request
        .supplier
        .trim()
        .to_ascii_lowercase()
        .replace(' ', "-");
    let cure_window = request.cure_days.max(5);
    let data_use_weight = if amendment_text.contains("retention") || amendment_text.contains("data")
    {
        0.94
    } else {
        0.76
    };
    let notice_weight = if amendment_text.contains("notice") || amendment_text.contains("breach") {
        0.91
    } else {
        0.68
    };
    let policy_weight = if amendment_text.contains("policy") || amendment_text.contains("transfer")
    {
        0.89
    } else {
        0.61
    };
    let graph = ObligationGraph {
        nodes: vec![
            ContractNode {
                id: "agreement-scope".into(),
                kind: ContractNodeKind::SupplierBoundary,
                criticality: 72,
                cure_days: cure_window / 6 + 1,
            },
            ContractNode {
                id: supplier_id.clone(),
                kind: ContractNodeKind::ContractSupplier,
                criticality: request.affected_controls.max(25),
                cure_days: cure_window / 3 + 1,
            },
            ContractNode {
                id: "audit-retention".into(),
                kind: ContractNodeKind::DataUseObligation,
                criticality: 96,
                cure_days: cure_window / 2 + 1,
            },
            ContractNode {
                id: "incident-notice".into(),
                kind: ContractNodeKind::NoticeObligation,
                criticality: 78,
                cure_days: cure_window / 4 + 1,
            },
            ContractNode {
                id: "data-use-policy".into(),
                kind: ContractNodeKind::PolicyControl,
                criticality: 90,
                cure_days: cure_window / 5 + 1,
            },
            ContractNode {
                id: "subprocessor-notice".into(),
                kind: ContractNodeKind::Subprocessor,
                criticality: 64,
                cure_days: cure_window / 6 + 1,
            },
        ],
        edges: vec![
            ImpactEdge {
                from: supplier_id.clone(),
                to: "agreement-scope".into(),
                propagation: 0.82,
                required: true,
            },
            ImpactEdge {
                from: supplier_id.clone(),
                to: "audit-retention".into(),
                propagation: data_use_weight,
                required: true,
            },
            ImpactEdge {
                from: supplier_id.clone(),
                to: "incident-notice".into(),
                propagation: notice_weight,
                required: true,
            },
            ImpactEdge {
                from: supplier_id.clone(),
                to: "data-use-policy".into(),
                propagation: policy_weight,
                required: true,
            },
            ImpactEdge {
                from: "incident-notice".into(),
                to: "subprocessor-notice".into(),
                propagation: 0.74,
                required: false,
            },
            ImpactEdge {
                from: "data-use-policy".into(),
                to: "agreement-scope".into(),
                propagation: 0.58,
                required: true,
            },
            ImpactEdge {
                from: "audit-retention".into(),
                to: "subprocessor-notice".into(),
                propagation: 0.44,
                required: false,
            },
        ],
    };
    graph.validate()?;
    Ok(graph)
}

pub fn analyze_amendment(
    request: &AmendmentRequest,
) -> Result<AmendmentImpactAnalysis, DomainError> {
    let graph = obligation_graph_for_request(request)?;
    let origin = request
        .supplier
        .trim()
        .to_ascii_lowercase()
        .replace(' ', "-");
    graph.propagate(&origin, 6, 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request(amendment: &str) -> AmendmentRequest {
        AmendmentRequest {
            supplier: "vectorcloud-ai".into(),
            amendment: amendment.into(),
            affected_controls: 68,
            cure_days: 45,
            reviewer: "procurement-counsel".into(),
        }
    }

    #[test]
    fn propagation_prioritizes_audit_retention_for_retention_change() {
        let analysis = analyze_amendment(&request("model retention and breach notice")).unwrap();
        assert!(analysis.affected_nodes >= 5);
        let durable = analysis
            .impacts
            .iter()
            .find(|impact| impact.node_id == "audit-retention")
            .unwrap();
        assert!(durable.probability > 0.9);
        assert!(analysis.remediation_burden_days > 45);
    }

    #[test]
    fn alternate_paths_keep_only_strongest_probability() {
        let graph = obligation_graph_for_request(&request("policy retention exception")).unwrap();
        let analysis = graph.propagate("vectorcloud-ai", 8, 0.01).unwrap();
        let entry = analysis
            .impacts
            .iter()
            .find(|impact| impact.node_id == "agreement-scope")
            .unwrap();
        assert!(entry.probability >= 0.82);
    }

    #[test]
    fn cut_candidates_are_sorted_by_reduced_exposure() {
        let graph = obligation_graph_for_request(&request("event stream partition")).unwrap();
        let candidates = graph.cut_candidates("vectorcloud-ai").unwrap();
        assert!(candidates.windows(2).all(|pair| pair[0].1 >= pair[1].1));
    }

    #[test]
    fn rejects_unknown_edge_endpoint() {
        let mut graph = obligation_graph_for_request(&request("database loss")).unwrap();
        graph.edges.push(ImpactEdge {
            from: "missing".into(),
            to: "agreement-scope".into(),
            propagation: 0.5,
            required: true,
        });
        assert!(
            matches!(graph.validate(), Err(DomainError::Integrity(message)) if message.contains("unknown"))
        );
    }

    #[test]
    fn cycles_do_not_loop_forever() {
        let mut graph = obligation_graph_for_request(&request("queue failure")).unwrap();
        graph.edges.push(ImpactEdge {
            from: "subprocessor-notice".into(),
            to: "vectorcloud-ai".into(),
            propagation: 0.9,
            required: true,
        });
        let analysis = graph.propagate("vectorcloud-ai", 20, 0.001).unwrap();
        assert!(analysis.longest_path < graph.nodes.len());
    }

    #[test]
    fn probability_floor_removes_weak_optional_paths() {
        let graph = obligation_graph_for_request(&request("minor latency")).unwrap();
        let strict = graph.propagate("vectorcloud-ai", 6, 0.7).unwrap();
        let permissive = graph.propagate("vectorcloud-ai", 6, 0.01).unwrap();
        assert!(strict.affected_nodes < permissive.affected_nodes);
    }
}
