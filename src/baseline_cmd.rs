use crate::baseline::{Baseline, BaselineManager};
use crate::baseline_checker::BaselineChecker;
use crate::port_scanner::{PortInfo, PortScanner};
use crate::state_store::StateStore;
use anyhow::Result;

pub enum BaselineCommand {
    Capture { label: String },
    Check,
    Show,
    Clear,
}

pub struct BaselineCli {
    manager: BaselineManager,
    scanner: PortScanner,
    checker: BaselineChecker,
}

impl BaselineCli {
    pub fn new(store: StateStore, scanner: PortScanner) -> Self {
        Self {
            manager: BaselineManager::new(store),
            scanner,
            checker: BaselineChecker::default(),
        }
    }

    pub fn run(&self, cmd: BaselineCommand) -> Result<String> {
        match cmd {
            BaselineCommand::Capture { label } => self.capture(label),
            BaselineCommand::Check => self.check(),
            BaselineCommand::Show => self.show(),
            BaselineCommand::Clear => self.clear(),
        }
    }

    fn capture(&self, label: String) -> Result<String> {
        let ports = self.scanner.scan()?;
        let baseline = Baseline::from_port_infos(&ports, &label);
        let count = baseline.ports.len();
        self.manager.save(&baseline)?;
        Ok(format!("Baseline '{}' captured with {} ports.", label, count))
    }

    fn check(&self) -> Result<String> {
        let baseline = self.manager.load()?.ok_or_else(|| {
            anyhow::anyhow!("No baseline found. Run 'baseline capture' first.")
        })?;
        let current = self.scanner.scan()?;
        let violation = self.checker.check(&baseline, &current);
        if violation.is_clean() {
            Ok("Baseline check passed. No violations detected.".to_string())
        } else {
            Ok(format!("Baseline violation: {}", violation.summary()))
        }
    }

    fn show(&self) -> Result<String> {
        match self.manager.load()? {
            None => Ok("No baseline stored.".to_string()),
            Some(b) => {
                let mut ports: Vec<u16> = b.ports.iter().copied().collect();
                ports.sort();
                Ok(format!("Baseline '{}' ({}): {:?}", b.label, b.created_at, ports))
            }
        }
    }

    fn clear(&self) -> Result<String> {
        self.manager.clear()?;
        Ok("Baseline cleared.".to_string())
    }
}
