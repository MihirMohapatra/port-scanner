# Async TCP Port Scanner (Rust)

A fast, asynchronous TCP port scanner built with Rust and tokio.

## Features

- **Async Concurrent Scanning**: Uses tokio async runtime for efficient concurrent port scanning
- **Configurable Concurrency**: Control max simultaneous connections
- **Configurable Timeout**: Set connection timeout in milliseconds per port
- **Port Range Support**: Scan single port, range (1-1024), comma-separated list, or "common" ports
- **Host Resolution**: Supports both IP addresses and hostnames

## Requirements

- Rust 1.56+
- Cargo (comes with Rust)

## Installation

```bash
git clone https://github.com/MihirMohapatra/port-scanner.git
cd port-scanner
cargo build --release
```

## Usage

### Basic Examples

Scan common ports on localhost:
```bash
cargo run -- --host 127.0.0.1 -p common
```

Scan specific ports:
```bash
cargo run -- --host example.com -p "80,443,8080"
```

Scan port range:
```bash
cargo run -- --host 192.168.1.1 -p "1-1024"
```

Show only open ports:
```bash
cargo run -- --host 192.168.1.1 -p "1-1000" --open-only
```

### Full Options

```bash
cargo run -- \
    --host 192.168.1.1 \
    --ports "1-1000" \
    --concurrency 100 \
    --timeout 1000 \
    --open-only
```

**Arguments:**
- `--host` - Target host IP or hostname (required)
- `-p, --ports` - Port specification: single port, range (1-1024), comma-separated (80,443), or "common" (default: common)
- `-c, --concurrency` - Max concurrent connections (default: 100)
- `-t, --timeout` - Connection timeout in milliseconds (default: 1000)
- `--open-only` - Show only open ports

## Testing

Test on localhost (no ports likely open):
```bash
cargo run -- --host 127.0.0.1 -p "22,80,443"
```

Test port range:
```bash
cargo run -- --host 127.0.0.1 -p "1-100" -c 50
```

Test hostname resolution:
```bash
cargo run -- --host google.com -p "80,443"
```

Run tests:
```bash
cargo test
```

Build release binary:
```bash
cargo build --release
./target/release/async-port-scanner --host localhost -p common
```

## Push to GitHub

```bash
# Initialize git if not already initialized
git init

# Add remote
git remote add origin https://github.com/MihirMohapatra/port-scanner.git

# Add files
git add .

# Commit
git commit -m "Initial commit: Rust async TCP port scanner"

# Push
git push -u origin main
```

## Performance Notes

- Higher concurrency = faster scanning but more system resource usage
- Lower timeout = faster results but may miss slow-responding ports
- Default concurrency of 100 works well for most use cases
- For scanning many ports, consider batches to avoid network congestion

## License

MIT