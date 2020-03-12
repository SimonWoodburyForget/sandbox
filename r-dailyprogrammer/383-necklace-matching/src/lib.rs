use std::collections::HashMap;

const enable1: &str = ""; //include_str!("inputs/enable1.txt");

/// Prints some data related to the input.
fn analyze(input: &str) {
    println!("kilobytes {} (bytes {})", input.len() / 1000, input.len());
    let words: Vec<&str> = enable1.trim().split("\n").collect();
    println!("words {}", words.len());

    println!(
        "max word len {}",
        words.iter().map(|w| w.len()).max().unwrap()
    );
    println!(
        "min word len {}",
        words.iter().map(|w| w.len()).min().unwrap()
    );

    let mut chars: HashMap<char, u32> = HashMap::new();
    for c in enable1.chars().filter(|&c| c != '\n') {
        let mut count = chars.entry(c).or_insert(0);
        *count += 1;
    }
    let mut chars = chars.into_iter().collect::<Vec<(char, u32)>>();
    chars.sort();
    println!("chars {:?}", chars);
}

/// Calculates a normalized starting index of a necklace.
///
/// - abcba < bcdaa
/// - abcba > aabcb
///
fn normalize(a: &str) -> usize {
    for c in a.chars() {}
    1
}

pub mod slicer {

    #[inline(always)]
    pub fn is_necklace(a: &str, b: &str) -> bool {
        let check = |rotation| {
            let a = (&a[rotation..], &a[..rotation]);
            let b = (&b[..a.0.len()], &b[a.0.len()..]);
            a == b
        };

        let len = a.len();
        len == b.len() && ((0..len).any(check) || len == 0)
    }

    #[test]
    pub fn test() {
        assert!(is_necklace("nicole", "icolen"));
        assert!(is_necklace("ab", "ba"));
        assert!(!is_necklace("x", "xx"));
        assert!(!is_necklace("", "x"));
        assert!(!is_necklace("xx", "x"));
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
    }
}

pub mod simple {
    use std::collections::VecDeque;

    #[inline(always)]
    pub fn is_necklace(a: &str, b: &str) -> bool {
        a.len() == b.len() && [a, a].concat().contains(b)
    }

    #[test]
    pub fn test() {
        assert!(is_necklace("nicole", "icolen"));
        assert!(is_necklace("ab", "ba"));
        assert!(!is_necklace("x", "xx"));
        assert!(!is_necklace("", "x"));
        assert!(!is_necklace("xx", "x"));
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
    }
}

pub mod manual {
    pub fn is_necklace(rhs: &str, lhs: &str) -> bool {
        if rhs.len() != lhs.len() {
            return false;
        } else if rhs.is_empty() {
            return true;
        }
        'outer: for offset in 0..rhs.len() {
            for (r, l) in rhs[offset..]
                .chars()
                .chain(rhs[..offset].chars())
                .zip(lhs.chars())
            {
                if r != l {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }

    #[test]
    pub fn test() {
        assert!(is_necklace("nicole", "icolen"));
        assert!(is_necklace("ab", "ba"));
        assert!(!is_necklace("x", "xx"));
        assert!(!is_necklace("", "x"));
        assert!(!is_necklace("xx", "x"));
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
    }
}
