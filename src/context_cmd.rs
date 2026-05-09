//! CLI sub-command: `portwatch context <port> [--proto tcp|udp]`
//! Prints enriched context for a given port.

use crate::context_builder::ContextBuilder;

#[derive(Debug)]
pub struct ContextArgs {
    pub port: u16,
    pub protocol: String,
    pub use_proc: bool,
}

impl Default for ContextArgs {
    fn default() -> Self {
        Self {
            port: 80,
            protocol: "tcp".into(),
            use_proc: true,
        }
    }
}

pub fn run(args: &ContextArgs) {
    let builder = ContextBuilder::new().with_proc(args.use_proc);
    let ctx = builder.build(args.port, &args.protocol);

    println!("Port Context");
    println!("  Port     : {}", ctx.port);
    println!("  Protocol : {}", ctx.protocol);
    println!("  Hostname : {}", ctx.hostname);
    println!(
        "  PID      : {}",
        ctx.pid.map(|p| p.to_string()).as_deref().unwrap_or("-")
    );
    println!(
        "  Process  : {}",
        ctx.process_name.as_deref().unwrap_or("-")
    );
    println!(
        "  User     : {}",
        ctx.username.as_deref().unwrap_or("-")
    );
    if !ctx.extra.is_empty() {
        println!("  Extra:");
        let mut keys: Vec<&String> = ctx.extra.keys().collect();
        keys.sort();
        for k in keys {
            println!("    {} = {}", k, ctx.extra[k]);
        }
    }
    println!("  Summary  : {}", ctx.summary());
}
