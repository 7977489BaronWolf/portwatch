//! CLI sub-commands for managing port filter rules (list, add, remove).

use crate::filter::{FilterRule, FilterSet, Protocol};
use std::path::Path;

const FILTER_FILE: &str = "portwatch_filters.json";

pub fn load_filter_set(path: &Path) -> anyhow::Result<FilterSet> {
    if !path.exists() {
        return Ok(FilterSet::default());
    }
    let data = std::fs::read_to_string(path)?;
    let set: FilterSet = serde_json::from_str(&data)?;
    Ok(set)
}

pub fn save_filter_set(path: &Path, set: &FilterSet) -> anyhow::Result<()> {
    let data = serde_json::to_string_pretty(set)?;
    std::fs::write(path, data)?;
    Ok(())
}

pub fn cmd_list_filters() -> anyhow::Result<()> {
    let path = Path::new(FILTER_FILE);
    let set = load_filter_set(path)?;
    if set.rules.is_empty() {
        println!("No filter rules defined.");
        return Ok(());
    }
    println!("{:<6} {:<20} {:<8} {}", "#", "Port/Range", "Proto", "Comment");
    for (i, rule) in set.rules.iter().enumerate() {
        let port_str = if let Some(p) = rule.port {
            p.to_string()
        } else if let Some((lo, hi)) = rule.port_range {
            format!("{}-{}", lo, hi)
        } else {
            "*".to_string()
        };
        let proto_str = match rule.protocol {
            Protocol::Tcp => "tcp",
            Protocol::Udp => "udp",
            Protocol::Any => "any",
        };
        let comment = rule.comment.as_deref().unwrap_or("");
        println!("{:<6} {:<20} {:<8} {}", i + 1, port_str, proto_str, comment);
    }
    Ok(())
}

pub fn cmd_add_filter(
    port: Option<u16>,
    range: Option<(u16, u16)>,
    proto: Protocol,
    comment: Option<String>,
) -> anyhow::Result<()> {
    let path = Path::new(FILTER_FILE);
    let mut set = load_filter_set(path)?;
    set.rules.push(FilterRule { port, port_range: range, protocol: proto, comment });
    save_filter_set(path, &set)?;
    println!("Filter rule added.");
    Ok(())
}

pub fn cmd_remove_filter(index: usize) -> anyhow::Result<()> {
    let path = Path::new(FILTER_FILE);
    let mut set = load_filter_set(path)?;
    if index == 0 || index > set.rules.len() {
        anyhow::bail!("Index {} out of range (1..={}).", index, set.rules.len());
    }
    set.rules.remove(index - 1);
    save_filter_set(path, &set)?;
    println!("Filter rule {} removed.", index);
    Ok(())
}
