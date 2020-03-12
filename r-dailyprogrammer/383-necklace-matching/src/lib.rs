use std::collections::HashMap;

/// Prints some data related to the input.
fn analyze(input: &str) {
    println!("kilobytes {} (bytes {})", input.len() / 1000, input.len());
    let words: Vec<&str> = input.trim().split("\n").collect();
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
    for c in input.chars().filter(|&c| c != '\n') {
        let mut count = chars.entry(c).or_insert(0);
        *count += 1;
    }
    let mut chars = chars.into_iter().collect::<Vec<(char, u32)>>();
    chars.sort();
    println!("chars {:?}", chars);
}

pub mod slicer {

    #[inline(always)]
    pub fn is_necklace(a: &str, b: &str) -> bool {
        let check = |(rotation, _)| {
            let a = (&a[rotation..], &a[..rotation]);
            let b = (&b[..a.0.len()], &b[a.0.len()..]);
            a == b
        };

        a.len() == b.len() && (a.len() == 0 || a.char_indices().any(check))
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
        'outer: for (offset, _) in rhs.char_indices() {
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
