use crate::benchmark::{benchmark_dft_fft, print_benchmark_csv};

mod utils;
mod types;
mod fourier;
mod benchmark;

fn main() {
    let res = benchmark_dft_fft(3,16,5, 12345);
    print_benchmark_csv(&res);
}
