//! CLI command handlers for the tag management feature.

use crate::tag::TagStore;

#[derive(Debug)]
pub enum TagCmd {
    Add { port: u16, tag: String },
    Remove { port: u16, tag: String },
    List { port: u16 },
    Find { tag: String },
    Clear { port: u16 },
}

pub struct TagCmdHandler {
    pub store: TagStore,
}

impl TagCmdHandler {
    pub fn new() -> Self {
        TagCmdHandler {
            store: TagStore::new(),
        }
    }

    pub fn execute(&mut self, cmd: TagCmd) -> TagCmdResult {
        match cmd {
            TagCmd::Add { port, tag } => {
                let inserted = self.store.add_tag(port, &tag);
                if inserted {
                    TagCmdResult::Ok(format!("Tag '{}' added to port {}.", tag, port))
                } else {
                    TagCmdResult::Ok(format!("Tag '{}' already present on port {}.", tag, port))
                }
            }
            TagCmd::Remove { port, tag } => {
                let removed = self.store.remove_tag(port, &tag);
                if removed {
                    TagCmdResult::Ok(format!("Tag '{}' removed from port {}.", tag, port))
                } else {
                    TagCmdResult::Err(format!("Tag '{}' not found on port {}.", tag, port))
                }
            }
            TagCmd::List { port } => {
                let tags = self.store.get_tags(port);
                if tags.is_empty() {
                    TagCmdResult::Ok(format!("No tags for port {}.", port))
                } else {
                    TagCmdResult::Ok(format!("Port {} tags: {}.", port, tags.join(", ")))
                }
            }
            TagCmd::Find { tag } => {
                let ports = self.store.ports_with_tag(&tag);
                if ports.is_empty() {
                    TagCmdResult::Ok(format!("No ports tagged '{}'.", tag))
                } else {
                    let list: Vec<String> = ports.iter().map(|p| p.to_string()).collect();
                    TagCmdResult::Ok(format!("Ports with tag '{}': {}.", tag, list.join(", ")))
                }
            }
            TagCmd::Clear { port } => {
                self.store.clear_port(port);
                TagCmdResult::Ok(format!("All tags cleared for port {}.", port))
            }
        }
    }
}

impl Default for TagCmdHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TagCmdResult {
    Ok(String),
    Err(String),
}

impl TagCmdResult {
    pub fn message(&self) -> &str {
        match self {
            TagCmdResult::Ok(m) | TagCmdResult::Err(m) => m,
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, TagCmdResult::Ok(_))
    }
}
