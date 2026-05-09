//! CLI subcommands for managing routing rules.

use crate::routing::{Router, RoutingRule};

pub fn handle_routing_command(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("list") => cmd_list(),
        Some("add") => cmd_add(&args[1..]),
        Some("remove") => cmd_remove(&args[1..]),
        Some("test") => cmd_test(&args[1..]),
        _ => print_usage(),
    }
}

fn print_usage() {
    println!("Usage: portwatch routing <list|add|remove|test>");
    println!("  list                                  List all routing rules");
    println!("  add <name> <severity|*> <tag|*> <ch>  Add a routing rule");
    println!("  remove <name>                         Remove a routing rule by name");
    println!("  test <severity> [tags...]             Test which channels would be triggered");
}

fn cmd_list() {
    // In production this would load from persistent config.
    println!("Routing rules (loaded from config):");
    println!("  (use 'portwatch routing add' to define rules)");
}

fn cmd_add(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: portwatch routing add <name> <severity|*> <tag|*> <channel,...>");
        return;
    }
    let name = &args[0];
    let severity = if args[1] == "*" { None } else { Some(args[1].clone()) };
    let tag = if args[2] == "*" { None } else { Some(args[2].clone()) };
    let channels: Vec<String> = args[3].split(',').map(|s| s.trim().to_string()).collect();

    let rule = RoutingRule {
        name: name.clone(),
        match_severity: severity,
        match_tag: tag,
        channels,
    };
    println!("Would add routing rule: {:?}", rule);
    println!("(Persist to config file to make permanent)");
}

fn cmd_remove(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: portwatch routing remove <name>");
        return;
    }
    println!("Would remove routing rule '{}' from config.", args[0]);
}

fn cmd_test(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: portwatch routing test <severity> [tag...]");
        return;
    }
    let severity = &args[0];
    let tags: Vec<String> = args[1..].to_vec();

    // Build a demo router for illustration.
    let mut router = Router::new();
    router.add_rule(RoutingRule {
        name: "example".to_string(),
        match_severity: None,
        match_tag: None,
        channels: vec!["default".to_string()],
    });

    let channels = router.resolve(severity, &tags);
    if channels.is_empty() {
        println!("No channels matched for severity='{}' tags={:?}", severity, tags);
    } else {
        println!("Matched channels: {:?}", channels);
    }
}
