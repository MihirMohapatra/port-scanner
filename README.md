# Async TCP Port Scanner

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.56+-DEA584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Async-Tokio-2D3239?style=for-the-badge&logo=tokio&logoColor=white" alt="Tokio">
  <img src="https://img.shields.io/badge/Performance-Concurrent-4CAF50?style=for-the-badge" alt="Performance">
</p>

<p align="center">
  A fast, asynchronous TCP port scanner built with Rust and Tokio. Scan ports efficiently with configurable concurrency and timeout settings.
</p>

---

## Features

- **Async Concurrent Scanning**: Uses Tokio async runtime for efficient concurrent port scanning
- **Configurable Concurrency**: Control max simultaneous connections (default: 100)
- **Configurable Timeout**: Set connection timeout in milliseconds per port (default: 1000ms)
- **Port Range Support**: Scan single port, range (1-1024), comma-separated list, or "common" ports
- **Host Resolution**: Supports both IP addresses and hostnames
- **Filtered Output**: Show only open ports with `--open-only` flag

---

## Quick Start

### 1. Clone and Build

```bash
git clone https://github.com/MihirMohapatra/port-scanner.git
cd port-scanner
cargo build --release
```

### 2. Run the Scanner

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

---

## Installation

### Prerequisites

- **Rust 1.56+** - [Install Rust](https://www.rust-lang.org/tools/install)
- **Cargo** (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/MihirMohapatra/port-scanner.git
cd port-scanner

# Build release binary
cargo build --release

# Run
./target/release/async-port-scanner --host localhost -p common
```

### Pre-built Binary

After building, the binary will be at:
```bash
./target/release/async-port-scanner
```

---

## Usage

### Basic Examples

#### Scan common ports on localhost
```bash
cargo run -- --host 127.0.0.1 -p common
```

#### Scan specific ports
```bash
cargo run -- --host example.com -p "80,443,8080"
```

#### Scan port range
```bash
cargo run -- --host 192.168.1.1 -p "1-1024"
```

#### Show only open ports
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

### Arguments

| Argument | Short | Description | Default |
|----------|-------|-------------|---------|
| `--host` | - | Target host IP or hostname | (required) |
| `--ports` | `-p` | Port specification: single port, range (1-1024), comma-separated (80,443), or "common" | `common` |
| `--concurrency` | `-c` | Max concurrent connections | `100` |
| `--timeout` | `-t` | Connection timeout in milliseconds | `1000` |
| `--open-only` | - | Show only open ports | `false` |

### Port Specification

- **Single port**: `-p 80`
- **Port range**: `-p "1-1024"`
- **Multiple ports**: `-p "80,443,8080,9000"`
- **Common ports**: `-p common` (scans well-known ports)

---

## Examples

### Test on localhost

```bash
# Scan common ports
cargo run -- --host 127.0.0.1 -p common

# Scan specific ports
cargo run -- --host 127.0.0.1 -p "22,80,443"
```

### Test port range

```bash
# Scan 1-100 with 50 concurrent connections
cargo run -- --host 127.0.0.1 -p "1-100" -c 50
```

### Test hostname resolution

```bash
# Scan using hostname
cargo run -- --host google.com -p "80,443"
```

### Custom concurrency and timeout

```bash
# High concurrency for fast scanning
cargo run -- --host 192.168.1.1 -p "1-1000" -c 200 -t 500

# Lower concurrency for stability
cargo run -- --host 192.168.1.1 -p "1-1000" -c 50 -t 2000
```

---

## Performance Notes

| Setting | Low | Medium | High |
|---------|-----|--------|------|
| Concurrency | 50 | 100 | 200+ |
| Timeout (ms) | 500 | 1000 | 2000+ |

- **Higher concurrency** = faster scanning but more system resource usage
- **Lower timeout** = faster results but may miss slow-responding ports
- Default concurrency of 100 works well for most use cases
- For scanning many ports, consider batches to avoid network congestion
- Very high concurrency may cause false negatives due to network limits

---

## Testing

```bash
# Run all tests
cargo test

# Build release binary
cargo build --release

# Run the binary
./target/release/async-port-scanner --host localhost -p common
```

---

## Project Structure

```
port-scanner/
├── Cargo.toml          # Project manifest
├── src/
│   └── main.rs         # Main application code
├── .gitignore          # Git ignore rules
├── README.md           # This file
└── target/             # Build output (gitignored)
    └── release/
        └── async-port-scanner
```

---

## Troubleshooting

### Connection refused on all ports
```bash
# Check if host is reachable
ping 192.168.1.1

# Try with longer timeout
cargo run -- --host 192.168.1.1 -p "80,443" -t 3000
```

### Timeout errors
```bash
# Increase timeout for slow hosts
cargo run -- --host 192.168.1.1 -p "1-1000" -t 2000
```

### Too many false positives/negatives
```bash
# Reduce concurrency to avoid network congestion
cargo run -- --host 192.168.1.1 -p "1-1000" -c 50
```

### Hostname resolution failed
```bash
# Use IP address instead of hostname
cargo run -- --host 192.168.1.1 -p "80,443"
```

---

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## License

MIT License - see LICENSE file for details.

---

<p align="center">
  Built with Rust and Tokio
</p>