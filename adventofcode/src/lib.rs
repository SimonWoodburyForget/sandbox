extern crate rayon;
use rayon::prelude::*;

#[inline(always)]
pub fn solve(input: &str, max: &str) -> (u32, u32) {
    let mut input = unsafe { from_input_str_unchecked(input) };
    let max = unsafe { from_input_str_unchecked(max) };
    
    let mut task_a = 0;
    let mut task_b = 0;

    loop {
        if is_increase(&input) {
            if is_2_digit_same(&input) {
                task_a += 1;
            }

            if is_2_digit_same_advanced(&input) {
                task_b += 1;
            }
        }

        incr(&mut input);

        if input > max {
            break;
        }
    }

    return (task_a, task_b)
}

pub struct InputIterator {
    input: [i8; 6],
    max: [i8; 6],
}

impl Iterator for InputIterator {
    type Item = [i8; 6];

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let Self { input, max } = self;

        incr(input);
        if input > max {
            None
        } else {
            Some(*input)
        }
    }
}

#[inline(always)]
pub fn collect_range(input: [i8; 6], max: [i8; 6]) -> Vec<[i8; 6]> {
    InputIterator { input, max }.collect()
}

/// It should work, but it's not actually any faster.
#[inline(always)]
pub fn solve_par(input: &str, max: &str) -> (u32, u32) {
    let input = from_input_str_unchecked(input);
    let max = from_input_str_unchecked(max);

    let data = collect_range(input, max);
    
    let (task_a, task_b) = data.into_par_iter().map(|input| {
        if is_increase(&input) {
            if is_2_digit_same(&input) {
                (1, 0);
            }
           
            if is_2_digit_same_advanced(&input) {
                (0, 1);
            }
        }

        (0, 0)
    }).reduce(|| (0, 0), |a, b| {
        (a.0 + b.0,
         a.1 + b.1)
    });
    
    return (task_a, task_b)
}


#[inline(always)]
pub fn incr_by_value(mut input: [i8; 6]) -> [i8; 6] {
    for i in (0..6).into_iter().rev() {
        input[i] += 1;
        
        if input[i] != 10 {
            return input;
        }
        
        match i {
            0 => input[i] = 0,
            _ => input[i] = input[i - 1],
        }
    }

    input
}

#[inline(always)]
pub fn incr(input: &mut [i8; 6]) {
    for i in (0..6).into_iter().rev() {
        input[i] += 1;
        
        if input[i] != 10 {
            return;
        }
        
        match i {
            0 => input[i] = 0,
            _ => input[i] = input[i - 1],
        }
    }
}


#[inline(always)]
pub fn is_2_digit_same(input: &[i8; 6]) -> bool {
    (0..5).any(|i| input[i] == input[i + 1])
}

#[inline(always)]
pub fn is_2_digit_same_advanced(input: &[i8; 6]) -> bool {
    (0..5).any(|i| match i {
        0 => (input[0] == input[1]) && (input[0] != input[2]),
        4 => (input[4] == input[5]) && (input[4] != input[3]),
        n => (input[n] == input[n + 1]) && (input[n] != input[n - 1]) && (input[n] != input[n + 2]),
    })
}

#[inline(always)]
pub fn is_increase(input: &[i8; 6]) -> bool {
    (0..5).all(|i| input[i] <= input[i + 1])
}

/// Note: doesn't check if digit is valid.
#[inline(always)]
pub fn from_input_str_unchecked(input: &str) -> [i8; 6] {
    let bytes = input.as_bytes();
    [
        bytes[0] as i8 - 49 ,
        bytes[1] as i8 - 49,
        bytes[2] as i8 - 49,
        bytes[3] as i8 - 49,
        bytes[4] as i8 - 49,
        bytes[5] as i8 - 49,
    ]
}

#[inline(always)]
pub fn from_input_i32(input: i32) -> [i8; 6] {
    [
        ((input / 100_000) % 10) as i8,
        ((input / 10_000) % 10) as i8,
        ((input / 1_000) % 10) as i8,
        ((input / 100) % 10) as i8,
        ((input / 10) % 10) as i8,
        (input % 10) as i8,
    ]
}

