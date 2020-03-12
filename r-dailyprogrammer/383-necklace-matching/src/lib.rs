use std::collections::HashMap;

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

pub mod slicer {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

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

    #[inline(always)]
    pub fn canonicalize_string(x: &str) -> String {
        x.char_indices()
            .map(|(rotation, _)| [&x[rotation..], &x[..rotation]])
            .max()
            .unwrap_or([x, ""])
            .concat()
    }

    #[inline(always)]
    pub fn is_necklace(a: &str, b: &str) -> bool {
        let check = |(rotation, _)| {
            let a = (&a[rotation..], &a[..rotation]);
            let b = (&b[..a.0.len()], &b[a.0.len()..]);
            a == b
        };
        a.len() == b.len() && (a.len() == 0 || a.char_indices().any(check))
    }

    #[derive(Debug)]
    pub struct Necklace<'a>(&'a str);

    impl Eq for Necklace<'_> {}
    impl PartialEq for Necklace<'_> {
        fn eq(&self, other: &Self) -> bool {
            let (a, b) = (self.0, other.0);
            let check = |(rotation, _)| {
                let a = (&a[rotation..], &a[..rotation]);
                let b = (&b[..a.0.len()], &b[a.0.len()..]);
                a == b
            };
            a.len() == b.len() && (a.len() == 0 || a.char_indices().any(check))
        }
    }

    impl Hash for Necklace<'_> {
        fn hash<H: Hasher>(&self, h: &mut H) {
            let x = self.0;
            let [a, b] = x
                .char_indices()
                .map(|(rotation, _)| [&x[rotation..], &x[..rotation]])
                .max()
                .unwrap_or([x, ""]);
            h.write(a.as_bytes());
            h.write(b.as_bytes());
        }
    }

    #[inline(always)]
    pub fn find_the_four<'a>(words: &'a [&'a str]) -> Option<Vec<&'a str>> {
        let mut results = HashMap::with_capacity(words.len());
        for &word in words {
            let result = results.entry(Necklace(word)).or_insert(Vec::new());
            result.push(word);
            if result.len() == 4 {
                return Some(result.clone());
            }
        }
        None
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
        assert_eq!(Necklace("ab"), Necklace("ba"));
        assert_eq!(Necklace("aabaaaaabaab"), Necklace("aabaabaabaaa"));
        assert_eq!(Necklace("nicole"), Necklace("icolen"));
        assert_eq!(Necklace("nicole"), Necklace("icolen"));
        assert_eq!(Necklace("nicole"), Necklace("lenico"));
        assert_eq!(Necklace("aabaaaaabaab"), Necklace("aabaabaabaaa"));
        assert_eq!(Necklace("x"), Necklace("x"));
        assert_eq!(Necklace(""), Necklace(""));
        assert_ne!(Necklace("x"), Necklace("xx"));
        assert_ne!(Necklace("x"), Necklace(""));
        assert_ne!(Necklace("abc"), Necklace("cba"));
        assert_ne!(Necklace("xxyyy"), Necklace("xxxyy"));
        assert_ne!(Necklace("xyxxz"), Necklace("xxyxz"));
    }

    #[test]
    pub fn test_solution() {
        let v: Vec<&str> = include_str!("../inputs/enable1.txt")
            .trim()
            .split("\n")
            .collect();
        let result = find_the_four(&v);
        assert!(result.is_some());
    }
}

pub mod simple {
    use std::collections::HashMap;

    #[inline(always)]
    pub fn is_necklace(a: &str, b: &str) -> bool {
        a.len() == b.len() && [a, a].concat().contains(b)
    }

    #[inline(always)]
    pub fn canonicalize(x: &str) -> String {
        x.char_indices()
            .map(|(rotation, _)| [&x[rotation..], &x[..rotation]].concat())
            .min()
            .unwrap_or(x.to_string())
    }

    #[inline(always)]
    pub fn find_the_four(words: Vec<&str>) -> Option<[&str; 4]> {
        let mut results: HashMap<String, (usize, [&str; 4])> = HashMap::new();

        for word in words {
            let (idx, ref mut result) = results
                .entry(canonicalize(word))
                .or_insert(Default::default());

            result[*idx] = word;
            *idx += 1;
            if *idx == 4 {
                // println!("{:?}", result);
                return Some(*result);
            }
        }

        None
    }

    #[test]
    pub fn test_is_nacklace() {
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

    #[test]
    pub fn test_canon() {
        assert_eq!(canonicalize("pesto"), "estop");
        assert_eq!(canonicalize("ab"), canonicalize("ba"));
        assert_eq!(canonicalize("aabaaaaabaab"), canonicalize("aabaabaabaaa"));
        assert_eq!(canonicalize("nicole"), canonicalize("icolen"));
        assert_eq!(canonicalize("xxx"), canonicalize("xxx"));
        assert_eq!(canonicalize(""), canonicalize(""));
    }

    #[test]
    pub fn test_solution() {
        let result = find_the_four(
            include_str!("../inputs/enable1.txt")
                .trim()
                .split("\n")
                .collect(),
        );

        assert!(result.is_some());
    }
}