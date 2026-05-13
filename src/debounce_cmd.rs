//! CLI sub-commands for inspecting and managing the debounce state.

use crate::debounce::Debouncer;
use std::time::Duration;

/// Commands available for the debounce subsystem.
#[derive(Debug)]
pub enum DebounceCmd {
    /// Print how many keys are currently being tracked.
    Status,
    /// Reset the debounce state for a specific alert key.
    Reset { key: String },
    /// Clear all tracked debounce keys.
    Clear,
    /// Update the quiet window (in seconds).
    SetWindow { seconds: u64 },
}

/// Executes a `DebounceCmd` against the provided `Debouncer`.
/// Returns a human-readable result string.
pub fn run_cmd(debouncer: &mut Debouncer, cmd: DebounceCmd) -> String {
    match cmd {
        DebounceCmd::Status => {
            format!("debounce: {} key(s) tracked", debouncer.tracked_count())
        }
        DebounceCmd::Reset { key } => {
            debouncer.reset(&key);
            format!("debounce: reset key '{key}'")
        }
        DebounceCmd::Clear => {
            debouncer.clear();
            "debounce: all keys cleared".to_string()
        }
        DebounceCmd::SetWindow { seconds } => {
            // Reconstruct with new window; existing state is cleared as a side-effect.
            *debouncer = Debouncer::new(Duration::from_secs(seconds));
            format!("debounce: quiet window set to {seconds}s (state cleared)")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_debouncer() -> Debouncer {
        Debouncer::new(Duration::from_secs(30))
    }

    #[test]
    fn status_shows_count() {
        let mut d = make_debouncer();
        d.should_emit("k1");
        let msg = run_cmd(&mut d, DebounceCmd::Status);
        assert!(msg.contains('1'));
    }

    #[test]
    fn reset_cmd_allows_re_emit() {
        let mut d = make_debouncer();
        d.should_emit("port:80");
        run_cmd(&mut d, DebounceCmd::Reset { key: "port:80".into() });
        assert!(d.should_emit("port:80"));
    }

    #[test]
    fn clear_cmd_empties_state() {
        let mut d = make_debouncer();
        d.should_emit("a");
        d.should_emit("b");
        run_cmd(&mut d, DebounceCmd::Clear);
        assert_eq!(d.tracked_count(), 0);
    }

    #[test]
    fn set_window_resets_state() {
        let mut d = make_debouncer();
        d.should_emit("z");
        assert_eq!(d.tracked_count(), 1);
        run_cmd(&mut d, DebounceCmd::SetWindow { seconds: 10 });
        assert_eq!(d.tracked_count(), 0);
    }
}
