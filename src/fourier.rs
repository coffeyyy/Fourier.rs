use crate::types::Complex;
use crate::utils::{bit_reverse_copy, is_pow_of_two, twiddle};

pub fn dft(signal: &[Complex]) -> Vec<Complex> {
    let n = signal.len();
    let mut out = vec![Complex::new(0.0, 0.0); n];

    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for t in 0..n {
            let w = twiddle(k * t, n);
            sum = sum.add(&signal[t].mult(&w));
        }
        out[k] = sum;
    }

    out
}

pub fn fft(signal: Vec<Complex>) -> Vec<Complex> {
    let n = signal.len();
    assert!(is_pow_of_two(n), "FFT length must be a power of two!");

    let mut a = bit_reverse_copy(signal);

    let mut m = 2;

    while m <= n {
        let wm = twiddle(1, m);

        for k in (0..n).step_by(m) {
            let mut w = Complex::new(1.0, 1.0);
            for j in 0..(m / 2) {
                let t = w.mult(&a[k + j + m / 2]);
                let u = a[k + j];
                a[k + j] = u.add(&t);
                a[k + j + m / 2] = u.sub(t);

                w = w.mult(&wm);
            }
        }
        m *= 2;
    }

    a
}

