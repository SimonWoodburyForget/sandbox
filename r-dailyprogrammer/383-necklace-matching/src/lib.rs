use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[cfg(test)]
mod tests;

use num_bigint::BigUint;
// use num_integer::Integer;
use num_traits::{
    // cast::FromPrimitive,
    Pow,
    Zero,
};
use std::iter::Sum;

// pub trait Count: Integer + FromPrimitive + Pow<u32, Output = Self> + Sum + Copy {}
// impl Count for u128 {}

/// represents a fixed range of primes
pub struct Primes {
    /// ordered vector of primes
    numbers: Vec<usize>,

    /// sieve of prime numbers
    is_prime: Vec<bool>,

    /// maximum number tested
    n: usize,
}

impl Primes {
    /// Computes primes within range to n.
    pub fn sieve_erato(n: usize) -> Self {
        let mut is_prime = vec![true; n];
        // set 0, 1 to false
        is_prime.iter_mut().take(2).for_each(|x| *x = false);

        for i in 0..(n as f64).sqrt() as usize + 1 {
            if is_prime[i] {
                is_prime[i * i..n]
                    .iter_mut()
                    .step_by(i)
                    .for_each(|is_p| *is_p = false)
            }
        }

        let numbers = is_prime
            .iter()
            .enumerate()
            .filter_map(|(p, &is_p)| if is_p { Some(p) } else { None })
            .collect();

        Primes {
            numbers,
            n,
            is_prime,
        }
    }

    /// Iterator of primes relativistic to `n`.
    pub fn relative(&self, n: usize) -> impl Iterator<Item = &usize> {
        debug_assert!(n < self.n);
        // minor optimization for known primes; reduces average
        // runtime by ~%10 on primes within range of `0..10_000`
        if self.is_prime[n] {
            let idx = self.numbers.binary_search(&n).unwrap();
            &self.numbers[idx..idx + 1]
        } else {
            &self.numbers
        }
        .iter()
        .take_while(move |&&p| p <= n)
        .filter(move |&&p| n % p == 0)
    }

    /// Euler's totient function.
    pub fn phi(&self, n: usize) -> usize {
        let p1: usize = self.relative(n).map(|p| p - 1).product();
        let p: usize = self.relative(n).product();
        n * p1 / p
    }

    /// Return count of `k`-ary necklace of length `n` as `u128`.
    pub fn necklaces(&self, k: usize, n: usize) -> u128 {
        let k = k as u128;
        let range = 1..(n as f64).sqrt() as usize + 1;
        let nums = range.filter(|x| n % x == 0).map(|x| {
            let (a, b) = (x, n / x);
            let div_a = self.phi(a) as u128 * k.pow(b as u32);
            let div_b = self.phi(b) as u128 * k.pow(a as u32);
            (div_a + if a != b { div_b } else { 0 }) as u128
        });
        nums.sum::<u128>() / n as u128
    }

    /// Return count of `k`-ary necklace of length `n` as `BigUint`.
    pub fn necklaces_big(&self, k: usize, n: usize) -> BigUint {
        let k: BigUint = k.into();
        let range = 1..(n as f64).sqrt() as usize + 1;
        let nums = range.filter(|x| n % x == 0).map(|x| {
            let (a, b) = (x, n / x);
            let div_a = self.phi(a) * k.pow(b);
            let div_b = self.phi(b) * k.pow(a);
            div_a + if a != b { div_b } else { Zero::zero() }
        });
        nums.sum::<BigUint>() / n
    }

    // /// Return count of `k`-ary necklace of length `n` as `BigUint`.
    // pub fn necklaces_generic<T: Count>(&self, k: usize, n: usize) -> T {
    //     let k = T::from_usize(k).unwrap();
    //     let range = 1..(n as f64).sqrt() as usize + 1;
    //     let nums = range.filter(|x| n % x == 0).map(|x| {
    //         let (a, b) = (x, n / x);
    //         let div_a = T::from_usize(self.phi(a)).unwrap() * k.pow(b as u32);
    //         let div_b = T::from_usize(self.phi(b)).unwrap() * k.pow(a as u32);
    //         div_a + if a != b { div_b } else { Zero::zero() }
    //     });
    //     nums.sum::<T>() / T::from_usize(n).unwrap()
    // }
}

pub fn find_the_four_counters<'a>(words: &'a [&'a str]) -> Option<Vec<&'a str>> {
    // find one solution
    let mut counters = HashMap::with_capacity(words.len());
    let mut solution = None;
    for &word in words {
        // words smaller then 4 have no solution
        if word.len() < 4 {
            continue;
        }
        let counter = counters.entry(Necklace::new(word)).or_insert(0);
        *counter += 1;
        if *counter == 4 {
            solution = Some(word);
            break;
        }
    }

    // find other solutions
    if let Some(solution_word) = solution {
        let mut solutions = Vec::with_capacity(4);
        let rotation = Necklace::new(solution_word).rotate();
        for word in rotation {
            let word = word.to_string();
            if let Ok(x) = words.binary_search(&word.as_str()) {
                solutions.push(words[x]);
            }
        }
        Some(solutions)
    } else {
        None
    }
}

type Slices<'a> = (&'a str, &'a str);
#[inline(always)]
fn flip((a, b): Slices<'_>) -> Slices<'_> {
    (b, a)
}

/// Calculates rotation from canonicalized form.
pub fn canonicalize_rotation(x: &str) -> usize {
    x.char_indices()
        .map(|(rotation, _)| flip(x.split_at(rotation)))
        .max()
        .unwrap_or((x, ""))
        .1
        .len()
}

/// Represents the word with a rotation to it's canonical form.
#[derive(Debug, Clone, Copy)]
pub struct Necklace<'a> {
    word: &'a str,
    rotation: usize,
}

impl<'a> Necklace<'a> {
    pub fn new(word: &'a str) -> Self {
        Self {
            word,
            rotation: canonicalize_rotation(word),
        }
    }

    /// Slices the word to it's canonical form.
    fn slices(&self) -> Slices<'a> {
        let Self { word, rotation } = self;
        flip(word.split_at(*rotation))
    }

    /// Iterates slices with respect to canonical rotation.
    fn iter_slices(&self) -> impl Iterator<Item = char> + 'a {
        let (a, b) = self.slices();
        a.chars().chain(b.chars())
    }

    /// Returns the rotation iterator. -- Iterates through the rotated forms of a necklace,
    /// starting at the current rotation +1 and ending before the current rotation.
    fn rotate(&self) -> impl Iterator<Item = Necklace<'a>> {
        let word = self.word;
        let init_rotation = self.rotation;
        let mut rotation = 0;
        std::iter::from_fn(move || {
            rotation += 1;
            if rotation <= word.len() {
                let rotation = (rotation + init_rotation) % word.len();
                Some(Necklace { word, rotation })
            } else {
                None
            }
        })
    }
}

impl Ord for Necklace<'_> {
    /// Compares the laxial ordering of the canonical form to another.
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter_slices().cmp(other.iter_slices())
    }
}

impl PartialOrd for Necklace<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Necklace<'_> {}
impl PartialEq for Necklace<'_> {
    /// Checks whether the other necklace is of the same canonical form.
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Hash for Necklace<'_> {
    /// Hashes the canonical form of the word.
    fn hash<H: Hasher>(&self, h: &mut H) {
        let (a, b) = self.slices();
        h.write(a.as_bytes());
        h.write(b.as_bytes());
    }
}

impl ToString for Necklace<'_> {
    /// Returns the canonical form as a string.
    fn to_string(&self) -> String {
        self.iter_slices().collect()
    }
}

/// Checks if two strings are part of the same necklace.
#[inline(always)]
pub fn is_necklace(a: &str, b: &str) -> bool {
    let check = |(rotation, _)| b.split_at(a.len() - rotation) == flip(a.split_at(rotation));
    a.len() == b.len() && (a.len() == 0 || a.char_indices().any(check))
}

/// Prints some data related to the input.
pub fn analyze(input: &str) {
    println!("kilobytes {} (bytes {})", input.len() / 1000, input.len());
    let words: Vec<&str> = input.trim().split("\n").collect();
    println!("words {}", words.len());

    println!(
        "max word len {}",
        words.iter().map(|w| w.len()).max().unwrap()
    );
    println!(
        "avg word len {}",
        words.iter().map(|w| w.len()).sum::<usize>() / words.len()
    );
    println!(
        "min word len {}",
        words.iter().map(|w| w.len()).min().unwrap()
    );

    let mut chars: HashMap<char, u32> = HashMap::new();
    for c in input.chars().filter(|&c| c != '\n') {
        let count = chars.entry(c).or_insert(0);
        *count += 1;
    }
    let mut chars = chars.into_iter().collect::<Vec<(char, u32)>>();
    chars.sort();
    println!("chars {:?}", chars);
}

#[test]
pub fn order() {
    assert!(Necklace::new("ab") == Necklace::new("ba"));
}

#[test]
#[rustfmt::skip]
pub fn rotation() {
    let mut x = Necklace { word: "abc", rotation: 0 }.rotate();
    assert_eq!(x.next(), Some(Necklace { word: "abc", rotation: 1 }));
    assert_eq!(x.next(), Some(Necklace { word: "abc", rotation: 2 }));
    assert_eq!(x.next(), Some(Necklace { word: "abc", rotation: 0 }));
    assert_eq!(x.next(), None);
    
    let mut x = Necklace { word: "abc", rotation: 1 }.rotate();
    assert_eq!(x.next(), Some(Necklace { word: "abc", rotation: 2 }));
    assert_eq!(x.next(), Some(Necklace { word: "abc", rotation: 0 }));
    assert_eq!(x.next(), Some(Necklace { word: "abc", rotation: 1 }));
    assert_eq!(x.next(), None);
}

#[test]
pub fn test() {
    assert_eq!(is_necklace("nicole", "icolen"), true);
    assert_eq!(is_necklace("nicole", "lenico"), true);
    assert_eq!(is_necklace("nicole", "coneli"), false);
    assert_eq!(is_necklace("aabaaaaabaab", "aabaabaabaaa"), true);
    assert_eq!(is_necklace("abc", "cba"), false);
    assert_eq!(is_necklace("xxyyy", "xxxyy"), false);
    assert_eq!(is_necklace("xyxxz", "xxyxz"), false);
    assert_eq!(is_necklace("x", "x"), true);
    assert_eq!(is_necklace("x", "xx"), false);
    assert_eq!(is_necklace("x", ""), false);
    assert_eq!(is_necklace("", ""), true);
    assert!(is_necklace("nicole", "icolen"));
    assert!(is_necklace("ab", "ba"));
    assert!(!is_necklace("x", "xx"));
    assert!(!is_necklace("", "x"));
    assert!(!is_necklace("xx", "x"));
}

#[test]
pub fn test_eq() {
    assert_eq!(Necklace::new("ab"), Necklace::new("ba"));
    assert_eq!(Necklace::new("aabaaaaabaab"), Necklace::new("aabaabaabaaa"));
    assert_eq!(Necklace::new("nicole"), Necklace::new("icolen"));
    assert_eq!(Necklace::new("nicole"), Necklace::new("icolen"));
    assert_eq!(Necklace::new("nicole"), Necklace::new("lenico"));
    assert_eq!(Necklace::new("aabaaaaabaab"), Necklace::new("aabaabaabaaa"));
    assert_eq!(Necklace::new("x"), Necklace::new("x"));
    assert_eq!(Necklace::new(""), Necklace::new(""));
    assert_ne!(Necklace::new("x"), Necklace::new("xx"));
    assert_ne!(Necklace::new("x"), Necklace::new(""));
    assert_ne!(Necklace::new("abc"), Necklace::new("cba"));
    assert_ne!(Necklace::new("xxyyy"), Necklace::new("xxxyy"));
    assert_ne!(Necklace::new("xyxxz"), Necklace::new("xxyxz"));
}

#[test]
pub fn test_solution() {
    let v: Vec<&str> = include_str!("../inputs/enable1.txt")
        .trim()
        .split("\n")
        .collect();
    let mut result = find_the_four_counters(&v).unwrap();
    result.sort();

    assert_eq!(result, vec!["estop", "pesto", "stope", "topes"]);
}
