extern crate rayon;
use rayon::prelude::*;

#[inline]
pub fn step(r: &mut [f32], d: &[f32], n: usize) {
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
    r.chunks_mut(n).enumerate().for_each(step_row);
}

#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
