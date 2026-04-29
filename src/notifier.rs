use crate::config::{HookType, NotificationHook};
use crate::port_scanner::PortChange;

#[derive(Debug)]
pub enum NotifyError {
    WebhookFailed(String),
    CommandFailed(String),
    UnsupportedHookType(String),
}

impl std::fmt::Display for NotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotifyError::WebhookFailed(e) => write!(f, "Webhook notification failed: {}", e),
            NotifyError::CommandFailed(e) => write!(f, "Command notification failed: {}", e),
            NotifyError::UnsupportedHookType(t) => write!(f, "Unsupported hook type: {}", t),
        }
    }
}

pub struct Notifier {
    hooks: Vec<NotificationHook>,
}

impl Notifier {
    pub fn new(hooks: Vec<NotificationHook>) -> Self {
        Notifier { hooks }
    }

    pub fn notify_all(&self, changes: &[PortChange]) -> Vec<NotifyError> {
        if changes.is_empty() {
            return vec![];
        }
        let message = self.format_message(changes);
        let mut errors = vec![];
        for hook in &self.hooks {
            if let Err(e) = self.dispatch(hook, &message) {
                errors.push(e);
            }
        }
        errors
    }

    fn dispatch(&self, hook: &NotificationHook, message: &str) -> Result<(), NotifyError> {
        match hook.hook_type {
            HookType::Webhook => self.send_webhook(&hook.target, message),
            HookType::Command => self.run_command(&hook.target, message),
            HookType::Email => Err(NotifyError::UnsupportedHookType("email".to_string())),
        }
    }

    fn send_webhook(&self, url: &str, message: &str) -> Result<(), NotifyError> {
        let payload = format!("{{\"text\": \"{}\"}}", message.replace('"', "'"));
        let output = std::process::Command::new("curl")
            .args(["-s", "-X", "POST", "-H", "Content-Type: application/json", "-d", &payload, url])
            .output()
            .map_err(|e| NotifyError::WebhookFailed(e.to_string()))?;
        if !output.status.success() {
            return Err(NotifyError::WebhookFailed(format!("HTTP request failed for {}", url)));
        }
        Ok(())
    }

    fn run_command(&self, cmd: &str, message: &str) -> Result<(), NotifyError> {
        let status = std::process::Command::new("sh")
            .args(["-c", cmd])
            .env("PORTWATCH_MESSAGE", message)
            .status()
            .map_err(|e| NotifyError::CommandFailed(e.to_string()))?;
        if !status.success() {
            return Err(NotifyError::CommandFailed(format!("Command '{}' exited with non-zero status", cmd)));
        }
        Ok(())
    }

    fn format_message(&self, changes: &[PortChange]) -> String {
        let lines: Vec<String> = changes.iter().map(|c| format!("{}", c)).collect();
        format!("[portwatch] Port changes detected: {}", lines.join("; "))
    }
}
