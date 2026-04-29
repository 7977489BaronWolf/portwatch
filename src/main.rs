mod config;
mod config_tests;
mod notifier;
mod notifier_tests;
mod port_scanner;
mod port_scanner_tests;
mod state_store;
mod state_store_tests;

use std::time::{SystemTime, UNIX_EPOCH};

use config::Config;
use notifier::Notifier;
use port_scanner::PortScanner;
use state_store::{PortState, StateStore};

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn run(config: &Config, store: &StateStore, scanner: &PortScanner, notifier: &Notifier) {
    let open_ports = scanner.scan();
    let now = current_timestamp();
    let new_state = PortState::new(open_ports, now);

    match store.load() {
        Ok(Some(prev_state)) => {
            let diff = prev_state.diff(&new_state);
            if !diff.is_empty() {
                notifier.notify(&diff);
            }
        }
        Ok(None) => {
            println!("[portwatch] No previous state found. Establishing baseline.");
        }
        Err(e) => {
            eprintln!("[portwatch] Failed to load state: {}", e);
        }
    }

    if let Err(e) = store.save(&new_state) {
        eprintln!("[portwatch] Failed to save state: {}", e);
    }
}

fn main() {
    let config = Config::load("portwatch.toml").unwrap_or_default();
    let store = StateStore::new(&config.state_file);
    let scanner = PortScanner::new(&config);
    let notifier = Notifier::new(&config);

    println!("[portwatch] Starting daemon (interval: {}s)", config.interval_secs);

    loop {
        run(&config, &store, &scanner, &notifier);
        std::thread::sleep(std::time::Duration::from_secs(config.interval_secs));
    }
}
