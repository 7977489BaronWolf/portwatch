mod config;
mod notifier;
mod port_scanner;

#[cfg(test)]
mod config_tests;
#[cfg(test)]
mod notifier_tests;
#[cfg(test)]
mod port_scanner_tests;

use config::Config;
use notifier::Notifier;
use port_scanner::PortScanner;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

const DEFAULT_CONFIG_PATH: &str = "/etc/portwatch/config.toml";

fn main() {
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_CONFIG_PATH.to_string());

    let config = match Config::load(&config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[portwatch] Failed to load config: {}", e);
            eprintln!("[portwatch] Using default configuration.");
            Config::default()
        }
    };

    println!(
        "[portwatch] Starting. Scan interval: {}s",
        config.scan_interval_secs
    );

    let notifier = Notifier::new(config.notification_hooks.clone());
    let scanner = PortScanner::new();
    let watch_set: HashSet<u16> = config.ports_to_watch.iter().cloned().collect();

    let mut previous = scanner.scan(&watch_set);
    println!("[portwatch] Initial scan complete. {} ports open.", previous.len());

    loop {
        thread::sleep(Duration::from_secs(config.scan_interval_secs));

        let current = scanner.scan(&watch_set);
        let changes = scanner.diff(&previous, &current);

        if !changes.is_empty() {
            println!("[portwatch] Detected {} change(s).", changes.len());
            let errors = notifier.notify_all(&changes);
            for err in &errors {
                eprintln!("[portwatch] Notification error: {}", err);
            }
        }

        previous = current;
    }
}
