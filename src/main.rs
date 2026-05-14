use clap::Parser;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::timeout;

#[derive(Parser, Debug)]
#[command(name = "port-scanner")]
#[command(about = "Async TCP port scanner", long_about = None)]
struct Args {
    #[arg(long, help = "Target host IP or hostname")]
    host: String,

    #[arg(short, long, help = "Port specification: single port, range (1-1024), comma-separated, or 'common'")]
    ports: String,

    #[arg(short, long, default_value = "100", help = "Max concurrent connections")]
    concurrency: usize,

    #[arg(short, long, default_value = "1000", help = "Connection timeout in milliseconds")]
    timeout: u64,

    #[arg(long, help = "Show only open ports")]
    open_only: bool,
}

const COMMON_PORTS: &[u16] = &[
    20, 21, 22, 23, 25, 53, 67, 68, 69, 80, 110, 119, 123, 135, 139, 143, 161, 162, 179, 194,
    389, 443, 445, 465, 514, 515, 587, 636, 993, 995, 1080, 1433, 1521, 1723, 2049, 2082,
    2083, 2086, 2087, 2095, 2096, 3306, 3389, 5432, 5900, 8080, 8443, 8888, 9090,
];

fn parse_ports(ports_spec: &str) -> Result<Vec<u16>, String> {
    if ports_spec.to_lowercase() == "common" {
        return Ok(COMMON_PORTS.to_vec());
    }

    let mut ports = Vec::new();

    for part in ports_spec.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            if range.len() != 2 {
                return Err(format!("Invalid port range: {}", part));
            }
            let start: u16 = range[0]
                .parse()
                .map_err(|_| format!("Invalid port: {}", range[0]))?;
            let end: u16 = range[1]
                .parse()
                .map_err(|_| format!("Invalid port: {}", range[1]))?;
            if start > end {
                return Err(format!("Invalid range: {}-{}", start, end));
            }
            for p in start..=end {
                ports.push(p);
            }
        } else {
            let port: u16 = part
                .parse()
                .map_err(|_| format!("Invalid port: {}", part))?;
            ports.push(port);
        }
    }

    ports.sort();
    ports.dedup();
    Ok(ports)
}

fn resolve_host(host: &str) -> Result<String, String> {
    let addr_str = if host.contains(':') {
        host.to_string()
    } else {
        format!("{}:0", host)
    };

    match addr_str.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                Ok(addr.ip().to_string())
            } else {
                Err(format!("Could not resolve host: {}", host))
            }
        }
        Err(e) => Err(format!("Failed to resolve host '{}': {}", host, e)),
    }
}

async fn scan_port(
    host: String,
    port: u16,
    timeout_ms: u64,
    semaphore: Arc<Semaphore>,
) -> (u16, bool) {
    let _permit = semaphore.acquire().await.unwrap();

    let addr: SocketAddr = match format!("{}:{}", host, port).parse() {
        Ok(a) => a,
        Err(_) => return (port, false),
    };

    let connect = tokio::net::TcpStream::connect(addr);
    let result = timeout(Duration::from_millis(timeout_ms), connect).await;

    match result {
        Ok(Ok(_stream)) => (port, true),
        _ => (port, false),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let resolved_host = resolve_host(&args.host)?;
    let ports = parse_ports(&args.ports)?;
    let total_ports = ports.len();

    println!("Scanning target: {}", resolved_host);
    println!("Ports: {} ({} total)", args.ports, total_ports);
    println!("Concurrency: {}, Timeout: {}ms", args.concurrency, args.timeout);
    println!();
    println!("Port    Status");
    println!("------  ------");

    let semaphore = Arc::new(Semaphore::new(args.concurrency));
    let timeout_ms = args.timeout;

    let mut handles = Vec::new();

    for port in ports {
        let host = resolved_host.clone();
        let handle = tokio::spawn(scan_port(host, port, timeout_ms, semaphore.clone()));
        handles.push(handle);
    }

    let mut open_count = 0;
    let mut closed_count = 0;

    for handle in handles {
        let (port, is_open) = handle.await.unwrap();
        if is_open {
            open_count += 1;
            if !args.open_only || (args.open_only && is_open) {
                println!("{:>5}   OPEN", port);
            }
        } else {
            closed_count += 1;
            if !args.open_only {
                println!("{:>5}   CLOSED", port);
            }
        }
    }

    println!();
    println!("Results: {} open, {} closed", open_count, closed_count);

    Ok(())
}