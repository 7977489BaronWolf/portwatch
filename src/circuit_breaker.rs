//! Circuit breaker for notification channels to prevent alert storms.

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    failure_threshold: u32,
    success_threshold: u32,
    timeout: Duration,
    state: CircuitState,
    failure_count: u32,
    success_count: u32,
    last_failure: Option<Instant>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, success_threshold: u32, timeout: Duration) -> Self {
        Self {
            failure_threshold,
            success_threshold,
            timeout,
            state: CircuitState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure: None,
        }
    }

    pub fn state(&self) -> &CircuitState {
        &self.state
    }

    pub fn is_open(&self) -> bool {
        match self.state {
            CircuitState::Open => {
                if let Some(last) = self.last_failure {
                    last.elapsed() < self.timeout
                } else {
                    true
                }
            }
            _ => false,
        }
    }

    pub fn allow_request(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if let Some(last) = self.last_failure {
                    if last.elapsed() >= self.timeout {
                        self.state = CircuitState::HalfOpen;
                        self.success_count = 0;
                        return true;
                    }
                }
                false
            }
            CircuitState::HalfOpen => true,
        }
    }

    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.success_threshold {
                    self.state = CircuitState::Closed;
                    self.failure_count = 0;
                    self.success_count = 0;
                }
            }
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            _ => {}
        }
    }

    pub fn record_failure(&mut self) {
        self.last_failure = Some(Instant::now());
        match self.state {
            CircuitState::Closed => {
                self.failure_count += 1;
                if self.failure_count >= self.failure_threshold {
                    self.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                self.state = CircuitState::Open;
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct CircuitBreakerRegistry {
    breakers: HashMap<String, CircuitBreaker>,
    default_failure_threshold: u32,
    default_success_threshold: u32,
    default_timeout: Duration,
}

impl CircuitBreakerRegistry {
    pub fn new(failure_threshold: u32, success_threshold: u32, timeout: Duration) -> Self {
        Self {
            breakers: HashMap::new(),
            default_failure_threshold: failure_threshold,
            default_success_threshold: success_threshold,
            default_timeout: timeout,
        }
    }

    pub fn get_or_create(&mut self, channel: &str) -> &mut CircuitBreaker {
        let ft = self.default_failure_threshold;
        let st = self.default_success_threshold;
        let to = self.default_timeout;
        self.breakers
            .entry(channel.to_string())
            .or_insert_with(|| CircuitBreaker::new(ft, st, to))
    }

    pub fn allow_request(&mut self, channel: &str) -> bool {
        self.get_or_create(channel).allow_request()
    }

    pub fn record_success(&mut self, channel: &str) {
        self.get_or_create(channel).record_success();
    }

    pub fn record_failure(&mut self, channel: &str) {
        self.get_or_create(channel).record_failure();
    }

    pub fn status(&self) -> HashMap<String, String> {
        self.breakers
            .iter()
            .map(|(k, v)| (k.clone(), format!("{:?}", v.state())))
            .collect()
    }
}
