//! CLI subcommand handlers for snapshot management.

use std::path::PathBuf;

use crate::port_scanner::scan_ports;
use crate::snapshot::{Snapshot, SnapshotStore};

pub fn cmd_snapshot_take(store_dir: &PathBuf, label: Option<String>) -> anyhow::Result<()> {
    let ports = scan_ports()?;
    let snap = Snapshot::new(ports, label.clone());
    let store = SnapshotStore::new(store_dir);
    store.save(&snap)?;
    println!(
        "Snapshot {} taken at {} with {} ports{}",
        snap.id,
        snap.timestamp,
        snap.ports.len(),
        label.map(|l| format!(" [{}]", l)).unwrap_or_default()
    );
    Ok(())
}

pub fn cmd_snapshot_list(store_dir: &PathBuf) -> anyhow::Result<()> {
    let store = SnapshotStore::new(store_dir);
    let snapshots = store.list()?;
    if snapshots.is_empty() {
        println!("No snapshots found.");
        return Ok(());
    }
    println!("{:<20} {:<12} {:<10} {}", "ID", "TIMESTAMP", "PORTS", "LABEL");
    println!("{}", "-".repeat(60));
    for s in &snapshots {
        println!(
            "{:<20} {:<12} {:<10} {}",
            s.id,
            s.timestamp,
            s.ports.len(),
            s.label.as_deref().unwrap_or("-")
        );
    }
    Ok(())
}

pub fn cmd_snapshot_diff(store_dir: &PathBuf, id_a: u64, id_b: u64) -> anyhow::Result<()> {
    let store = SnapshotStore::new(store_dir);
    let a = store.load(id_a)?;
    let b = store.load(id_b)?;
    let map_a = a.port_map();
    let map_b = b.port_map();

    let mut added: Vec<u16> = map_b.keys().filter(|p| !map_a.contains_key(p)).copied().collect();
    let mut removed: Vec<u16> = map_a.keys().filter(|p| !map_b.contains_key(p)).copied().collect();
    added.sort();
    removed.sort();

    println!("Diff snapshot {} -> {}:", id_a, id_b);
    if added.is_empty() && removed.is_empty() {
        println!("  No changes.");
    }
    for port in &added {
        println!("  + port {}", port);
    }
    for port in &removed {
        println!("  - port {}", port);
    }
    Ok(())
}

pub fn cmd_snapshot_delete(store_dir: &PathBuf, id: u64) -> anyhow::Result<()> {
    let store = SnapshotStore::new(store_dir);
    store.delete(id)?;
    println!("Snapshot {} deleted.", id);
    Ok(())
}
