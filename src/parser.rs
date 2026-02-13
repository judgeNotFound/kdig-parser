use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct KdigStats {
    pub query_time_ms: f64,
    pub response_size_bytes: u32,
    pub server: String,
    pub port: u16,
    pub protocol: String,
}

pub fn parse_kdig_file(path: &Path) -> Result<Option<KdigStats>> {
    let content = fs::read_to_string(path)?;

    // Compile regex patterns
    let received_regex = Regex::new(r";;\s+Received\s+(\d+)\s+B")?;
    let from_regex = Regex::new(r";;\s+From\s+([^@]+)@(\d+)\(([^)]+)\)\s+in\s+([\d.]+)\s+ms")?;

    // Extract data from file content
    let mut response_size: Option<u32> = None;
    let mut server: Option<String> = None;
    let mut port: Option<u16> = None;
    let mut protocol: Option<String> = None;
    let mut query_time: Option<f64> = None;

    for line in content.lines() {
        // Match response size
        if let Some(caps) = received_regex.captures(line) {
            if let Ok(size) = caps[1].parse::<u32>() {
                response_size = Some(size);
            }
        }

        // Match server, port, protocol, and query time
        if let Some(caps) = from_regex.captures(line) {
            server = Some(caps[1].trim().to_string());
            if let Ok(p) = caps[2].parse::<u16>() {
                port = Some(p);
            }
            protocol = Some(caps[3].trim().to_string());
            if let Ok(qt) = caps[4].parse::<f64>() {
                query_time = Some(qt);
            }
        }
    }

    // Check if we have all required fields
    match (query_time, response_size, server, port, protocol) {
        (Some(qt), Some(rs), Some(srv), Some(prt), Some(proto)) => Ok(Some(KdigStats {
            query_time_ms: qt,
            response_size_bytes: rs,
            server: srv,
            port: prt,
            protocol: proto,
        })),
        _ => Ok(None), // Not a valid kdig stats file or missing data
    }
}
