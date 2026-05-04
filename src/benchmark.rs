//! Module for the DIRAC Benchmark 2012 (DB12) logic.

use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Number of iterations corresponding to 1kHS2k (250 HS06 seconds).
const ITERATIONS: u64 = 1000 * 1000 * 12; // 12M iterations (ajusté pour Rust)
/// Calibration factor (from Python version).
const CALIBRATION: f64 = 250.0;

/// Result of a single benchmark run.
#[derive(Debug, Clone, serde::Serialize)]
pub struct BenchmarkResult {
    pub cpu_time: f64,      // CPU time in seconds
    pub wall_time: f64,     // Wall-clock time in seconds
    pub norm: f64,          // Normalized score (DB12 units)
    pub unit: String,       // Unit ("DB12")
}

/// Runs a single DB12 benchmark iteration.
///
/// # Arguments
/// * `iterations` - Number of benchmark iterations to run.
/// * `correction` - Whether to apply a correction factor.
///
/// # Returns
/// `BenchmarkResult` with CPU time, wall time, and normalized score.
pub fn single_dirac_benchmark(iterations: u64, correction: bool) -> BenchmarkResult {
    let normal = Normal::new(10.0, 1.0).unwrap();
    let mut rng = rand::thread_rng();

    let mut m1 = 0.0;
    let mut m2 = 0.0;
    let mut p1 = 0.0;
    let mut p2 = 0.0;

    // Warm-up iteration (ignored)
    for _ in 0..ITERATIONS {
        let t1 = normal.sample(&mut rng);
        m1 += t1;
        m2 += t1 * t1;
        p1 += t1;
        p2 += t1 * t1;
    }

    // Start timing
    let start_cpu = cpu_time();
    let start_wall = Instant::now();

    // Actual benchmark iterations
    for _ in 0..iterations {
        for _ in 0..ITERATIONS {
            let t1 = normal.sample(&mut rng);
            m1 += t1;
            m2 += t1 * t1;
            p1 += t1;
            p2 += t1 * t1;
        }
    }

    let end_cpu = cpu_time();
    let end_wall = Instant::now();

    let cpu_time = end_cpu - start_cpu;
    let wall_time = end_wall.duration_since(start_wall).as_secs_f64();

    // Calculate normalized score
    let norm = CALIBRATION * iterations as f64 / cpu_time;

    // Apply correction if needed
    let norm = if correction {
        apply_norm_correction(norm)
    } else {
        norm
    };

    BenchmarkResult {
        cpu_time,
        wall_time,
        norm,
        unit: "DB12".to_string(),
    }
}

/// Gets the current CPU time (user + system) for the current process.
/// Equivalent to `os.times()[0] + os.times()[1]` in Python.
fn cpu_time() -> f64 {
    unsafe {
        let mut tms = libc::tms {
            tms_utime: 0,
            tms_stime: 0,
            tms_cutime: 0,
            tms_cstime: 0,
        };
        libc::times(&mut tms);
        (tms.tms_utime + tms.tms_stime) as f64 / libc::sysconf(libc::_SC_CLK_TCK) as f64
    }
}

/// Applies a correction factor to the normalized score.
/// In Rust, this may not be necessary if performance is consistent.
fn apply_norm_correction(norm: f64) -> f64 {
    // TODO: Implement correction logic if needed (e.g., for compatibility with Python scores).
    // For now, return the raw norm (Rust should be stable across versions).
    log::warn!("Correction factors are not yet implemented for Rust. Returning raw norm.");
    norm
}