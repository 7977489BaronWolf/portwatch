use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    pub fn from_score(score: u32) -> Self {
        match score {
            0..=25 => Priority::Low,
            26..=50 => Priority::Medium,
            51..=75 => Priority::High,
            _ => Priority::Critical,
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Priority::Low => 10,
            Priority::Medium => 40,
            Priority::High => 65,
            Priority::Critical => 90,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Critical => "critical",
        }
    }

    pub fn is_actionable(&self) -> bool {
        matches!(self, Priority::High | Priority::Critical)
    }

    pub fn escalate(&self) -> Self {
        match self {
            Priority::Low => Priority::Medium,
            Priority::Medium => Priority::High,
            Priority::High => Priority::Critical,
            Priority::Critical => Priority::Critical,
        }
    }

    pub fn downgrade(&self) -> Self {
        match self {
            Priority::Low => Priority::Low,
            Priority::Medium => Priority::Low,
            Priority::High => Priority::Medium,
            Priority::Critical => Priority::High,
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

impl TryFrom<&str> for Priority {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "critical" => Ok(Priority::Critical),
            other => Err(format!("Unknown priority: {}", other)),
        }
    }
}
