//! CLI subcommands for managing portwatch plugins.

use crate::plugin::{PluginHook, PluginMeta, PluginRegistry};

/// List all registered plugins, optionally filtered by hook.
pub fn cmd_list(registry: &PluginRegistry, hook_filter: Option<&str>) {
    if registry.count() == 0 {
        println!("No plugins registered.");
        return;
    }

    let filter = hook_filter.and_then(parse_hook);

    for hook in &[
        PluginHook::OnPortOpen,
        PluginHook::OnPortClose,
        PluginHook::OnBaselineViolation,
        PluginHook::OnAlert,
    ] {
        if let Some(ref f) = filter {
            if f != hook {
                continue;
            }
        }
        let plugins = registry.plugins_for_hook(hook);
        if !plugins.is_empty() {
            println!("[{}]", hook);
            for name in plugins {
                if let Some(meta) = registry.get(name) {
                    println!("  {} (v{})", meta.name, meta.version);
                }
            }
        }
    }
}

/// Show detailed info for a single plugin.
pub fn cmd_info(registry: &PluginRegistry, name: &str) {
    match registry.get(name) {
        Some(meta) => {
            println!("Plugin : {}", meta.name);
            println!("Version: {}", meta.version);
            println!("Hooks  : {}", meta.hooks.iter().map(|h| h.to_string()).collect::<Vec<_>>().join(", "));
        }
        None => eprintln!("Plugin '{}' not found.", name),
    }
}

/// Register a new plugin entry (used in tests / scripted setup).
pub fn cmd_register(registry: &mut PluginRegistry, name: &str, version: &str, hooks: Vec<&str>) {
    let parsed: Vec<PluginHook> = hooks.iter().filter_map(|h| parse_hook(h)).collect();
    if parsed.is_empty() {
        eprintln!("No valid hooks provided for plugin '{}'.", name);
        return;
    }
    registry.register(PluginMeta {
        name: name.to_string(),
        version: version.to_string(),
        hooks: parsed,
    });
    println!("Plugin '{}' registered successfully.", name);
}

fn parse_hook(s: &str) -> Option<PluginHook> {
    match s {
        "on_port_open" => Some(PluginHook::OnPortOpen),
        "on_port_close" => Some(PluginHook::OnPortClose),
        "on_baseline_violation" => Some(PluginHook::OnBaselineViolation),
        "on_alert" => Some(PluginHook::OnAlert),
        _ => None,
    }
}
