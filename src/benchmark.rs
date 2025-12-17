use crate::types::Complex;
use std::hint::black_box;
use std::time::Instant;

// --- simple PRNG (no rand crate needed) ---
#[derive(Clone)]
struct XorShift64 {
    state: u64,
}
impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_f64(&mut self) -> f64 {
        // uniform-ish in [0, 1)
        let u = self.next_u64() >> 11; // 53 bits
        (u as f64) * (1.0 / ((1u64 << 53) as f64))
    }

    fn next_f64_range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next_f64()
    }
}

fn random_complex_vec(n: usize, rng: &mut XorShift64) -> Vec<Complex> {
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        let re = rng.next_f64_range(-1.0, 1.0);
        let im = rng.next_f64_range(-1.0, 1.0);
        v.push(Complex::new(re, im));
    }
    v
}

/// Benchmark DFT vs FFT for sizes 2^min_pow .. 2^max_pow.
/// `trials` repeats each size and averages the time.
///
/// Returns Vec of (n, dft_avg_ms, fft_avg_ms)
pub fn benchmark_dft_fft(
    min_pow: u32,
    max_pow: u32,
    trials: usize,
    seed: u64,
) -> Vec<(usize, f64, f64)> {
    let mut rng = XorShift64::new(seed);
    let mut results = Vec::new();

    for p in min_pow..=max_pow {
        let n = 1usize << p;

        let mut dft_total_ms = 0.0;
        let mut fft_total_ms = 0.0;

        for _ in 0..trials {
            let input = random_complex_vec(n, &mut rng);

            // --- DFT timing ---
            let start = Instant::now();
            let out_dft = crate::fourier::dft(black_box(&input));
            let dft_ms = start.elapsed().as_secs_f64() * 1000.0;
            black_box(out_dft);
            dft_total_ms += dft_ms;

            // --- FFT timing ---
            let start = Instant::now();
            let out_fft = crate::fourier::fft(black_box(input.clone()));
            let fft_ms = start.elapsed().as_secs_f64() * 1000.0;
            black_box(out_fft);
            fft_total_ms += fft_ms;
        }

        let dft_avg = dft_total_ms / trials as f64;
        let fft_avg = fft_total_ms / trials as f64;

        results.push((n, dft_avg, fft_avg));
    }

    results
}

pub fn print_benchmark_csv(results: &[(usize, f64, f64)]) {
    println!("n,dft_ms,fft_ms");
    for (n, dft_ms, fft_ms) in results {
        println!("{},{},{}", n, dft_ms, fft_ms);
    }
}
