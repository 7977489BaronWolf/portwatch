//! Tag module: assign and manage user-defined tags on ports for grouping and filtering.

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagStore {
    /// Maps port number to a set of tag strings.
    tags: HashMap<u16, HashSet<String>>,
}

impl TagStore {
    pub fn new() -> Self {
        TagStore {
            tags: HashMap::new(),
        }
    }

    /// Add a tag to a port. Returns true if the tag was newly inserted.
    pub fn add_tag(&mut self, port: u16, tag: &str) -> bool {
        self.tags
            .entry(port)
            .or_insert_with(HashSet::new)
            .insert(tag.to_string())
    }

    /// Remove a tag from a port. Returns true if the tag existed.
    pub fn remove_tag(&mut self, port: u16, tag: &str) -> bool {
        if let Some(set) = self.tags.get_mut(&port) {
            let removed = set.remove(tag);
            if set.is_empty() {
                self.tags.remove(&port);
            }
            removed
        } else {
            false
        }
    }

    /// Get all tags for a port.
    pub fn get_tags(&self, port: u16) -> Vec<String> {
        self.tags
            .get(&port)
            .map(|s| {
                let mut v: Vec<String> = s.iter().cloned().collect();
                v.sort();
                v
            })
            .unwrap_or_default()
    }

    /// Return all ports that carry the given tag.
    pub fn ports_with_tag(&self, tag: &str) -> Vec<u16> {
        let mut ports: Vec<u16> = self
            .tags
            .iter()
            .filter(|(_, tags)| tags.contains(tag))
            .map(|(port, _)| *port)
            .collect();
        ports.sort();
        ports
    }

    /// Clear all tags for a port.
    pub fn clear_port(&mut self, port: u16) {
        self.tags.remove(&port);
    }

    /// Return a snapshot of the full tag map.
    pub fn snapshot(&self) -> HashMap<u16, Vec<String>> {
        self.tags
            .iter()
            .map(|(port, set)| {
                let mut v: Vec<String> = set.iter().cloned().collect();
                v.sort();
                (*port, v)
            })
            .collect()
    }
}

impl Default for TagStore {
    fn default() -> Self {
        Self::new()
    }
}
