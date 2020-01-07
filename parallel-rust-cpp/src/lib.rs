extern crate rayon;
use rayon::prelude::*;

#[inline]
pub fn step_v0a(r: &mut [f32], d: &[f32], n: usize) {
    for i in 0..n {
        for j in 0..n {
            let mut v = std::f32::INFINITY;
            for k in 0..n {
                let x = d[n * i + k];
                let y = d[n * k + j];
                let z = x + y;
                v = v.min(z);
            }
            r[n * i + j] = v;
        }
    }
}

#[inline]
pub fn step_v0b(r: &mut [f32], d: &[f32], n: usize) {
    let step_row = |(i, r_row): (usize, &mut [f32])| {
        for (j, res) in r_row.iter_mut().enumerate() {
            let mut v = std::f32::INFINITY;
            for k in 0..n {
                let x = d[n * i + k];
                let y = d[n * k + j];
                let z = x + y;
                v = v.min(z);
            }
            *res = v;
        }
    };
    // r.par_chunks_mut(n).enumerate().for_each(step_row);
    r.chunks_mut(n).enumerate().for_each(step_row);
}

#[inline]
pub fn step_v0c(r: &mut [f32], d: &[f32], n: usize) {
    let step_row = |(i, r_row): (usize, &mut [f32])| {
        for (j, res) in r_row.iter_mut().enumerate() {
            let mut v = std::f32::INFINITY;
            for k in 0..n {
                let x = d[n * i + k];
                let y = d[n * k + j];
                let z = x + y;
                v = v.min(z);
            }
            *res = v;
        }
    };
    r.par_chunks_mut(n).enumerate().for_each(step_row);
    // r.chunks_mut(n).enumerate().for_each(step_row);
}

#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_slow(n - 1) + fibonacci_slow(n - 2),
    }
}

pub fn fibonacci_fast(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}
