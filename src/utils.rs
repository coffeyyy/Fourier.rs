use crate::types::Complex;
use std::f64::consts::PI;



fn factorial(n: u64) -> f64 {
    (1..=n).fold(1.0, |acc, k| acc * (k as f64))
}

pub fn is_pow_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

fn reverse_bits(mut x: u32, bits: u32) -> u32 {
    let mut rev = 0;

    for _ in 0..bits {
        rev = (rev << 1) | (x & 1);
        x >>= 1;
    }

    rev
}

pub fn bit_reverse_copy(a: Vec<Complex>) -> Vec<Complex> {
    let n: usize = a.len();
    let mut output = vec![Complex::new(0.0, 0.0); n];

    let bits = n.trailing_zeros();

    for i in 0..n {
        let rev = reverse_bits(i as u32, bits) as usize;
        output[rev] = a[i];
    }

    output
}

fn cis(theta: f64) -> Complex {
    Complex::new(theta.cos(), theta.sin())
}

pub fn twiddle(k: usize, m: usize) -> Complex {
    
    let theta = -2.0 * PI * (k as f64) / (m as f64);
    cis(theta)
}