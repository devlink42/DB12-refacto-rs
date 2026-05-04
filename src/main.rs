//! DB12 Rust CLI.

use clap::{Parser, Subcommand};
use db12_rs::benchmark::single_dirac_benchmark;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
#[command(name = "db12-rs")]
#[command(about = "DIRAC Benchmark 2012 (DB12) in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Number of iterations to perform (default: 1).
    #[arg(long, default_value_t = 1)]
    iterations: u64,

    /// Output results to a JSON file.
    #[arg(long)]
    json: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a single benchmark (1 CPU core).
    Single,

    /// Run multiple benchmarks in parallel.
    Multiple {
        /// Number of copies to run in parallel.
        copies: usize,
    },

    /// Run benchmarks on all available CPU cores.
    Wholenode,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Single => {
            let res = single_dirac_benchmark(cli.iterations);
            vec![res]
        }
        Commands::Multiple { copies } => {
            (0..copies)
                .map(|_| single_dirac_benchmark(cli.iterations))
                .collect()
        }
        Commands::Wholenode => {
            let copies = num_cpus::get();
            (0..copies)
                .map(|_| single_dirac_benchmark(cli.iterations))
                .collect()
        }
    };

    // Print or save results
    if let Some(filename) = cli.json {
        let json = serde_json::to_string(&result).unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    } else {
        for (i, res) in result.iter().enumerate() {
            println!(
                "Copy {}: CPU={:.2}s, WALL={:.2}s, NORM={:.2} {}",
                i + 1,
                res.cpu_time,
                res.wall_time,
                res.norm,
                res.unit
            );
        }
        // Print statistics if multiple copies
        if result.len() > 1 {
            let norms: Vec<f64> = result.iter().map(|r| r.norm).collect();
            let sum: f64 = norms.iter().sum();
            let mean = sum / norms.len() as f64;
            let product: f64 = norms.iter().product();
            let geometric_mean = product.powf(1.0 / norms.len() as f64);
            let mut sorted = norms.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = sorted[sorted.len() / 2];

            println!(
                "\nStatistics: sum={:.2}, arithmetic_mean={:.2}, geometric_mean={:.2}, median={:.2}",
                sum, mean, geometric_mean, median
            );
        }
    }
}
