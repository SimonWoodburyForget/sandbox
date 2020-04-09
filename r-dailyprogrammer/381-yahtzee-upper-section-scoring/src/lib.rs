//! Nothing Here
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    num::ParseIntError,
};

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

    let wordset: HashSet<&str> = words.iter().cloned().collect();
    println!("eq words {:?}", wordset.len());
}

pub enum Error {
    ParseError,
    EmptyDice,
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Error::ParseError
    }
}

pub fn yahtzee_decode(dice: &str) -> Result<Option<u32>, ParseIntError> {
    let dice = dice
        .trim()
        .split("\n")
        .map(|x| x.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    let mut numbs = HashMap::new();
    dice.iter().for_each(|&x| *numbs.entry(x).or_insert(0) += 1);
    Ok(numbs.iter().map(|(&x, y)| x * y).max())
}

pub fn yahtzee_hashmap(dice: &[u32]) -> Option<u32> {
    let mut numbs = HashMap::new();
    dice.iter().for_each(|&x| *numbs.entry(x).or_insert(0) += 1);
    numbs.iter().map(|(&x, y)| x * y).max()
}

pub fn yahtzee_vec(dice: &[u32]) -> Option<u32> {
    let mut numbs: Vec<(u32, u32)> = Vec::new();
    for &x in dice {
        if let Ok(x) = numbs.binary_search_by(|y| y.0.cmp(&x)) {
            numbs[x].1 += 1;
        } else {
            numbs.push((x, 1));
            numbs.sort_by(|a, b| a.0.cmp(&b.0));
        }
    }
    numbs.iter().map(|(x, y)| x * y).max()
}

pub fn yahtzee_btree(dice: &[u32]) -> Option<u32> {
    let mut numbs = BTreeMap::new();
    dice.iter().for_each(|&x| *numbs.entry(x).or_insert(0) += 1);
    numbs.iter().map(|(&x, y)| x * y).max()
}

pub fn yahtzee_small(dice: [u32; 5]) -> u32 {
    let mut r = [0_u32; 6];
    dice.iter().for_each(|&x| r[x as usize - 1] += 1);
    r.iter()
        .enumerate()
        .map(|(x, &y)| (1 + x as u32) * y)
        .max()
        .unwrap()
}

mod tests {
    use super::*;
    fn ints() -> Vec<([u32; 5], u32)> {
        vec![
            ([2, 3, 5, 5, 6], 10),
            ([1, 1, 1, 1, 3], 4),
            ([1, 1, 1, 3, 3], 6),
            ([1, 2, 3, 4, 5], 5),
            ([6, 6, 6, 6, 6], 30),
        ]
    }

    #[test]
    fn test_yahtzee_hashmap() {
        for (a, b) in ints().iter() {
            assert_eq!(yahtzee_hashmap(a), Some(*b));
        }
    }

    #[test]
    fn test_yahtzee_small() {
        for (a, b) in ints().iter() {
            assert_eq!(yahtzee_small(*a), *b);
        }
    }

    #[test]
    fn test_yahtzee_vec() {
        for (a, b) in ints().iter() {
            assert_eq!(yahtzee_vec(a), Some(*b));
        }
    }

    #[test]
    fn test_yahtzee_btree() {
        for (a, b) in ints().iter() {
            assert_eq!(yahtzee_btree(a), Some(*b));
        }
    }
}
