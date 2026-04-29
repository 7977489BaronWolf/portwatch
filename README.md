# portwatch

A daemon that monitors open ports and alerts on unexpected changes with configurable notification hooks.

## Installation

```bash
cargo install portwatch
```

Or build from source:

```bash
cargo build --release && sudo cp target/release/portwatch /usr/local/bin/
```

## Usage

Start the daemon with a configuration file:

```bash
portwatch --config /etc/portwatch/config.toml
```

Example `config.toml`:

```toml
interval = 30  # seconds between scans

[hooks]
on_open  = "/usr/local/bin/notify-open.sh"
on_close = "/usr/local/bin/notify-close.sh"

[whitelist]
ports = [22, 80, 443]
```

When an unexpected port is detected, portwatch executes the configured hook script with the port number and event type as arguments:

```
/usr/local/bin/notify-open.sh 8080 open
```

### CLI Flags

| Flag | Description |
|------|-------------|
| `--config <path>` | Path to configuration file |
| `--once` | Run a single scan and exit |
| `--verbose` | Enable verbose logging |

## Contributing

Bug reports and pull requests are welcome. Please open an issue before submitting large changes.

## License

This project is licensed under the [MIT License](LICENSE).