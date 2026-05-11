//! CLI commands for inspecting circuit breaker state.

use crate::circuit_breaker::CircuitBreakerRegistry;
use std::time::Duration;

pub struct CircuitBreakerCmd {
    registry: CircuitBreakerRegistry,
}

impl CircuitBreakerCmd {
    pub fn new(failure_threshold: u32, success_threshold: u32, timeout_secs: u64) -> Self {
        Self {
            registry: CircuitBreakerRegistry::new(
                failure_threshold,
                success_threshold,
                Duration::from_secs(timeout_secs),
            ),
        }
    }

    pub fn run(&mut self, args: &[String]) -> Result<(), String> {
        match args.first().map(|s| s.as_str()) {
            Some("status") => self.cmd_status(),
            Some("reset") => {
                let channel = args.get(1).ok_or("Usage: reset <channel>".to_string())?;
                self.cmd_reset(channel)
            }
            Some("test") => {
                let channel = args.get(1).ok_or("Usage: test <channel>".to_string())?;
                self.cmd_test(channel)
            }
            _ => Err("Unknown command. Available: status, reset, test <channel>".to_string()),
        }
    }

    fn cmd_status(&self) -> Result<(), String> {
        let status = self.registry.status();
        if status.is_empty() {
            println!("No circuit breakers registered.");
        } else {
            println!("{:<20} {}", "Channel", "State");
            println!("{}", "-".repeat(35));
            let mut entries: Vec<_> = status.iter().collect();
            entries.sort_by_key(|(k, _)| k.as_str());
            for (channel, state) in entries {
                println!("{:<20} {}", channel, state);
            }
        }
        Ok(())
    }

    fn cmd_reset(&mut self, channel: &str) -> Result<(), String> {
        // Allow request to trigger state evaluation, then record success to close.
        let cb = self.registry.get_or_create(channel);
        *cb = crate::circuit_breaker::CircuitBreaker::new(
            3,
            2,
            Duration::from_secs(60),
        );
        println!("Circuit breaker for '{}' has been reset to Closed.", channel);
        Ok(())
    }

    fn cmd_test(&mut self, channel: &str) -> Result<(), String> {
        let allowed = self.registry.allow_request(channel);
        if allowed {
            println!("Channel '{}' is ALLOWING requests.", channel);
        } else {
            println!("Channel '{}' is BLOCKING requests (circuit open).", channel);
        }
        Ok(())
    }
}
