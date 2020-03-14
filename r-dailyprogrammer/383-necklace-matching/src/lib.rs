use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use rayon::prelude::*;

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

#[inline(always)]
pub fn canonicalize_hash(x: &str) -> u64 {
    let [a, b] = x
        .char_indices()
        .map(|(rotation, _)| [&x[rotation..], &x[..rotation]])
        .max()
        .unwrap_or([x, ""]);

    let mut h = DefaultHasher::new();
    h.write(a.as_bytes());
    h.write(b.as_bytes());
    h.finish()
}

#[inline(always)]
pub fn canonicalize_slices(x: &str) -> [&str; 2] {
    x.char_indices()
        .map(|(rotation, _)| [&x[rotation..], &x[..rotation]])
        .max()
        .unwrap_or([x, ""])
}

/// Calculates rotation from canonicalized form.
#[inline(always)]
pub fn canonicalize_rotation(x: &str) -> usize {
    x.char_indices()
        .map(|(rotation, _)| [&x[rotation..], &x[..rotation]])
        .max()
        .unwrap_or([x, ""])[1]
        .len()
}

/// Checks if two strings are part of the same necklace.
#[inline(always)]
pub fn is_necklace(a: &str, b: &str) -> bool {
    let check = |(rotation, _)| {
        let a = (&a[rotation..], &a[..rotation]);
        let b = (&b[..a.0.len()], &b[a.0.len()..]);
        a == b
    };
    a.len() == b.len() && (a.len() == 0 || a.char_indices().any(check))
}

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

    fn slices(&self) -> [&'a str; 2] {
        let Self { word, rotation } = self;
        [&word[*rotation..], &word[..*rotation]]
    }

    fn rotate(&self) -> Rotate<'a> {
        Rotate {
            necklace: *self,
            rotation: 0,
        }
    }
}

struct Rotate<'a> {
    necklace: Necklace<'a>,
    rotation: usize,
}

impl<'a> Iterator for Rotate<'a> {
    type Item = Necklace<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rotation += 1;
        if self.rotation <= self.necklace.word.len() {
            Some(Necklace {
                word: self.necklace.word,
                rotation: self.necklace.rotation + self.rotation,
            })
        } else {
            None
        }
    }
}

impl Ord for Necklace<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let [a, b] = self.slices();
        let x = a.chars().chain(b.chars());
        let [a, b] = other.slices();
        let y = a.chars().chain(b.chars());
        x.cmp(y)
    }
}

impl PartialOrd for Necklace<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Necklace<'_> {}
impl PartialEq for Necklace<'_> {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

impl Hash for Necklace<'_> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        let [a, b] = self.slices();
        h.write(a.as_bytes());
        h.write(b.as_bytes());
    }
}

impl ToString for Necklace<'_> {
    fn to_string(&self) -> String {
        self.slices().concat()
    }
}

#[inline(always)]
pub fn find_the_four_slow<'a>(words: &'a [&'a str]) -> Option<Vec<&'a str>> {
    let mut results = HashMap::with_capacity(words.len());
    for &word in words {
        let result = results.entry(Necklace::new(word)).or_insert(Vec::new());
        result.push(word);
        if result.len() == 4 {
            return Some(result.clone());
        }
    }
    None
}

#[inline(always)]
pub fn find_the_four_counters<'a>(words: &'a [&'a str]) -> Option<Vec<&'a str>> {
    // find one solution
    let mut counters = HashMap::with_capacity(words.len());
    let mut solution = None;
    for &word in words {
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
        let rotation = Necklace::new(solution_word)
            .rotate()
            .take(solution_word.len() - 1);
        for word in rotation {
            let word = word.to_string();
            if let Ok(x) = words.binary_search(&word.as_str()) {
                solutions.push(words[x]);
            }
        }
        solutions.push(solution_word);
        solutions.sort();
        Some(solutions)
    } else {
        None
    }
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
        assert_eq!(x.next(), Some(Necklace { word: "abc", rotation: 3 }));
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
    // result.sort();

    assert_eq!(result, vec!["estop", "pesto", "stope", "topes"]);
}
