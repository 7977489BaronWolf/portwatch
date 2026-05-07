//! Plugin system for extending portwatch with custom notification handlers.

use std::collections::HashMap;
use std::fmt;

/// Represents a plugin hook type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginHook {
    OnPortOpen,
    OnPortClose,
    OnBaselineViolation,
    OnAlert,
}

impl fmt::Display for PluginHook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginHook::OnPortOpen => write!(f, "on_port_open"),
            PluginHook::OnPortClose => write!(f, "on_port_close"),
            PluginHook::OnBaselineViolation => write!(f, "on_baseline_violation"),
            PluginHook::OnAlert => write!(f, "on_alert"),
        }
    }
}

/// Metadata describing a registered plugin.
#[derive(Debug, Clone)]
pub struct PluginMeta {
    pub name: String,
    pub version: String,
    pub hooks: Vec<PluginHook>,
}

/// Registry that holds all registered plugins and their hooks.
#[derive(Default)]
pub struct PluginRegistry {
    plugins: HashMap<String, PluginMeta>,
    hook_map: HashMap<PluginHook, Vec<String>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a plugin and index its hooks.
    pub fn register(&mut self, meta: PluginMeta) {
        for hook in &meta.hooks {
            self.hook_map
                .entry(hook.clone())
                .or_default()
                .push(meta.name.clone());
        }
        self.plugins.insert(meta.name.clone(), meta);
    }

    /// Return plugin names subscribed to a given hook.
    pub fn plugins_for_hook(&self, hook: &PluginHook) -> Vec<&str> {
        self.hook_map
            .get(hook)
            .map(|v| v.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    /// Returns the total number of registered plugins.
    pub fn count(&self) -> usize {
        self.plugins.len()
    }

    /// Retrieve metadata for a plugin by name.
    pub fn get(&self, name: &str) -> Option<&PluginMeta> {
        self.plugins.get(name)
    }

    /// Remove a plugin by name, cleaning up hook indexes.
    pub fn unregister(&mut self, name: &str) -> bool {
        if let Some(meta) = self.plugins.remove(name) {
            for hook in &meta.hooks {
                if let Some(list) = self.hook_map.get_mut(hook) {
                    list.retain(|n| n != name);
                }
            }
            true
        } else {
            false
        }
    }
}
