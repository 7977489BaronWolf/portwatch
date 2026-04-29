use crate::config::Config;
use crate::diff_engine::{compute_diff, format_diff_message};
use crate::notifier::Notifier;
use crate::port_scanner::scan_open_ports;
use crate::state_store::StateStore;
use std::thread;
use std::time::Duration;

pub struct Watcher {
    config: Config,
    store: StateStore,
    notifier: Notifier,
}

impl Watcher {
    pub fn new(config: Config, store: StateStore, notifier: Notifier) -> Self {
        Watcher { config, store, notifier }
    }

    pub fn run_once(&mut self) -> Result<bool, String> {
        let current_ports = scan_open_ports(&self.config)
            .map_err(|e| format!("Port scan failed: {}", e))?;

        let previous_ports = self.store.load().unwrap_or_default();
        let diff = compute_diff(&previous_ports, &current_ports);

        if !diff.is_empty() {
            let message = format_diff_message(&diff);
            self.notifier.notify(&message)
                .map_err(|e| format!("Notification failed: {}", e))?;
            self.store.save(&current_ports)
                .map_err(|e| format!("State save failed: {}", e))?;
            return Ok(true);
        }

        Ok(false)
    }

    pub fn run_loop(&mut self) {
        let interval = Duration::from_secs(self.config.poll_interval_secs);
        log::info!("Starting portwatch loop with {}s interval", interval.as_secs());

        loop {
            match self.run_once() {
                Ok(true) => log::info!("Port change detected and notified."),
                Ok(false) => log::debug!("No port changes."),
                Err(e) => log::error!("Watcher error: {}", e),
            }
            thread::sleep(interval);
        }
    }
}
