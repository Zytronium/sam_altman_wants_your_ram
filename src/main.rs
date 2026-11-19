use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::fs;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn format_bytes(bytes: u64) -> String {
    if bytes >= (1 << 30) {
        format!("{:.2} GB", bytes as f64 / (1 << 30) as f64)
    } else if bytes >= (1 << 20) {
        format!("{:.2} MB", bytes as f64 / (1 << 20) as f64)
    } else {
        format!("{:.2} KB", bytes as f64 / (1 << 10) as f64)
    }
}

fn round_up_to_physical_ram(mem_total_kb: u64) -> u64 {
    // Round up to the nearest common physical RAM size (in GB)
    // Common sizes: 4, 8, 16, 32, 64, 128, 256, 512, 1024 GB
    let gb = mem_total_kb as f64 / (1024.0 * 1024.0);
    let mut rounded = 4.0;
    while rounded < gb {
        rounded *= 2.0;
    }
    (rounded * 1024.0 * 1024.0 * 1024.0) as u64
}

fn get_meminfo() -> (u64, u64) {
    let content = fs::read_to_string("/proc/meminfo").unwrap();
    let mut total_kb: u64 = 0;
    let mut available: u64 = 0;
    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            total_kb = parts[1].parse::<u64>().unwrap();
        } else if line.starts_with("MemAvailable:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            available = parts[1].parse::<u64>().unwrap() * 1024;
        }
    }
    let total = round_up_to_physical_ram(total_kb);
    let used = total_kb * 1024 - available;
    (total, used)
}

fn main() {
    let (total_ram, used_ram) = get_meminfo();
    let usage_percent = (used_ram as f64 / total_ram as f64) * 100.0;

    println!();
    println!("  Total physical RAM:  {}", format_bytes(total_ram));
    println!("  Used RAM:            {}", format_bytes(used_ram));
    println!("  Memory utilization:  {:.1}%", usage_percent);
    println!();

    let mut rng = rand::rng();
    let delay: u64 = rng.random_range(300..800);
    thread::sleep(Duration::from_millis(delay));

    let total_size = used_ram;
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  [{elapsed_precise}] {bar:40} {percent:>3}% | {bytes_per_sec} | {wide_msg}")
            .unwrap()
            .progress_chars("█▓▒░ "),
    );
    pb.set_message("Transmitting memory dump to Sam Altman");

    let chunks = 120;
    let chunk_size = total_size / chunks;

    for i in 0..chunks {
        let increment = if i == chunks - 1 {
            total_size - pb.position()
        } else {
            chunk_size
        };
        pb.inc(increment);
        let ms = rng.random_range(8..45);
        thread::sleep(Duration::from_millis(ms));
    }

    pb.finish_with_message("Transmission complete");
    println!();
    print!("  Sending to Sam Altman...");
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(rng.random_range(400..900)));
    println!(" OK");
    println!();
}
