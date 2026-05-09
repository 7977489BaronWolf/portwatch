use crate::suppression_rule::{SuppressionCondition, SuppressionRule, SuppressionRuleStore};
use chrono::Duration;

#[derive(Debug)]
pub enum SuppressionRuleCmd {
    Add {
        id: String,
        name: String,
        reason: String,
        port: Option<u16>,
        port_range: Option<(u16, u16)>,
        protocol: Option<String>,
        tag: Option<String>,
        ttl_seconds: Option<i64>,
    },
    Remove {
        id: String,
    },
    List,
    Purge,
}

#[derive(Debug)]
pub enum SuppressionRuleCmdResult {
    Added(String),
    Removed(String),
    NotFound(String),
    Listed(Vec<String>),
    Purged(usize),
}

pub fn handle_suppression_rule_cmd(
    store: &mut SuppressionRuleStore,
    cmd: SuppressionRuleCmd,
) -> SuppressionRuleCmdResult {
    match cmd {
        SuppressionRuleCmd::Add {
            id, name, reason, port, port_range, protocol, tag, ttl_seconds,
        } => {
            let mut rule = SuppressionRule::new(&id, name, reason);
            if let Some(p) = port {
                rule = rule.with_condition(SuppressionCondition::ExactPort(p));
            }
            if let Some((lo, hi)) = port_range {
                rule = rule.with_condition(SuppressionCondition::PortRange(lo, hi));
            }
            if let Some(proto) = protocol {
                rule = rule.with_condition(SuppressionCondition::Protocol(proto));
            }
            if let Some(t) = tag {
                rule = rule.with_condition(SuppressionCondition::Tag(t));
            }
            if let Some(secs) = ttl_seconds {
                rule = rule.with_ttl(Duration::seconds(secs));
            }
            store.add(rule);
            SuppressionRuleCmdResult::Added(id)
        }
        SuppressionRuleCmd::Remove { id } => {
            if store.remove(&id).is_some() {
                SuppressionRuleCmdResult::Removed(id)
            } else {
                SuppressionRuleCmdResult::NotFound(id)
            }
        }
        SuppressionRuleCmd::List => {
            let entries = store
                .list_active()
                .iter()
                .map(|r| format!("[{}] {} — {}", r.id, r.name, r.reason))
                .collect();
            SuppressionRuleCmdResult::Listed(entries)
        }
        SuppressionRuleCmd::Purge => {
            let count = store.purge_expired();
            SuppressionRuleCmdResult::Purged(count)
        }
    }
}
