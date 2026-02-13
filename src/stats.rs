use crate::parser::KdigStats;
use std::collections::HashMap;

pub fn display_summary(stats: &[KdigStats]) {
    if stats.is_empty() {
        println!("No valid kdig statistics found.");
        return;
    }

    println!("\n=== Kdig Analysis Summary ===\n");
    println!("Total files analyzed: {}\n", stats.len());

    // Calculate query time statistics
    let mut query_times: Vec<f64> = stats.iter().map(|s| s.query_time_ms).collect();
    query_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min_time = query_times.iter().copied().fold(f64::INFINITY, f64::min);
    let max_time = query_times.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let avg_time: f64 = query_times.iter().sum::<f64>() / query_times.len() as f64;
    let median_time = if query_times.len() % 2 == 0 {
        (query_times[query_times.len() / 2 - 1] + query_times[query_times.len() / 2]) / 2.0
    } else {
        query_times[query_times.len() / 2]
    };

    println!("Query Time Statistics (ms):");
    println!("  Min:     {:.2}", min_time);
    println!("  Max:     {:.2}", max_time);
    println!("  Average: {:.2}", avg_time);
    println!("  Median:  {:.2}\n", median_time);

    // Calculate response size statistics
    let response_sizes: Vec<u32> = stats.iter().map(|s| s.response_size_bytes).collect();
    let min_size = *response_sizes.iter().min().unwrap();
    let max_size = *response_sizes.iter().max().unwrap();
    let avg_size: f64 = response_sizes.iter().sum::<u32>() as f64 / response_sizes.len() as f64;
    let total_size: u32 = response_sizes.iter().sum();

    println!("Response Size Statistics (bytes):");
    println!("  Min:     {}", min_size);
    println!("  Max:     {}", max_size);
    println!("  Average: {:.2}", avg_size);
    println!("  Total:   {}\n", total_size);

    // Collect unique servers
    let mut server_map: HashMap<String, usize> = HashMap::new();
    for stat in stats {
        let server_key = format!("{}:{}", stat.server, stat.port);
        *server_map.entry(server_key).or_insert(0) += 1;
    }

    println!("Unique Servers Queried:");
    let mut servers: Vec<(&String, &usize)> = server_map.iter().collect();
    servers.sort_by(|a, b| b.1.cmp(a.1));
    for (server, count) in servers {
        println!("  {} - {} queries", server, count);
    }
    println!();

    // Protocol distribution
    let mut protocol_map: HashMap<String, usize> = HashMap::new();
    for stat in stats {
        *protocol_map.entry(stat.protocol.clone()).or_insert(0) += 1;
    }

    println!("Protocol Distribution:");
    let mut protocols: Vec<(&String, &usize)> = protocol_map.iter().collect();
    protocols.sort_by(|a, b| b.1.cmp(a.1));
    for (protocol, count) in protocols {
        println!("  {}: {}", protocol, count);
    }
    println!();
}
