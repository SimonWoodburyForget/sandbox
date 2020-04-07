//! Daily programming challenge 380.

const ENABLE_1: &str = include_str!("enable1.txt");

pub mod bonus {
    use super::*;

    pub fn one() -> Morse {
        use std::collections::HashMap;
        let mut map: HashMap<Morse, u8> = HashMap::default();
        let mut result = None;
        let words = ENABLE_1.trim().split("\n");
        for x in words {
            let m: Morse = AlphaStr(x).into();
            let counter = map.entry(m).or_default();
            *counter += 1;
            if *counter == 13 {
                result = Some(m);
                break;
            }
        }
        assert_eq!(result, Some(Morse::from(AlphaStr("tsadi"))));
        assert_eq!(result, Some(Morse::from(MorseStr("-....--...."))));
        result.unwrap()
    }

    pub fn two() -> Morse {
        let a = Morse::from(MorseStr("---------------"));
        let words = ENABLE_1.trim().split("\n");
        let mut result = None;
        for x in words {
            let x = Morse::from(AlphaStr(x));
            if x.contains(a) {
                result = Some(x);
            }
        }
        result.unwrap()
    }

    pub fn three() -> Vec<&'static str> {
        let words = ENABLE_1.trim().split("\n");
        let mut results = Vec::new();
        for w in words {
            if w.len() == 21 {
                let x = Morse::from(AlphaStr(w));
                if x.balanced() {
                    results.push(w);
                    if results.len() == 2 {
                        break;
                    }
                }
            }
        }
        assert_eq!(results.len(), 2);
        results
    }

    pub fn four() -> Morse {
        let words = ENABLE_1.trim().split("\n");
        for w in words {
            if w.len() == 13 {
                let x = Morse::from(AlphaStr(w));
                if x == x.reversed() {
                    return x;
                }
            }
        }
        panic!();
    }

    pub fn five() {
        // use std::collections::HashSet;
        // let mut encodings_13: HashSet<u32> = Default::default();
        // let words = ENABLE_1.trim().split("\n");
        // for x in words {
        //     let m: Morse = AlphaStr(x).into();
        //     if m.len == 13 {
        //         encodings_13.insert(m.val as u32);
        //     }
        // }
        // let mut not_in_13: Vec<u32> = Default::default();
        // for x in 0b0000000000000..0b10000000000000 {
        //     if !encodings_13.contains(&x) {
        //         not_in_13.push(x);
        //     }
        // }
        // for x in not_in_13 {}

        // println!("{:?}", not_in_13);
        // panic!();
    }
}

pub use inner::*;
mod inner {

    /// Bit encoded morse code, each dot-or-dash is encoded as a 1 (dot) or 0 (dash),
    /// with the lenght of said sequence, which is done to make it as small as possible.
    #[derive(Debug, PartialEq, Default, Hash, Copy, Clone)]
    pub struct Morse {
        pub len: u8,
        pub val: u128,
    }

    impl Eq for Morse {}

    impl Morse {
        fn unchecked_push(&mut self, code: Morse) {
            self.len += code.len;
            self.val <<= code.len;
            self.val ^= code.val;
        }

        /// Push another morse on top of this one, like a letter.
        pub fn push(&mut self, code: Morse) {
            self.unchecked_push(code);
            assert!(self.len <= 128);
        }

        /// Push one bit onto the Morse code.
        pub fn push_bit(&mut self, bit: bool) {
            self.unchecked_push(Self {
                len: 1,
                val: bit as _,
            })
        }

        /// Checks if `Morse` contains another `Morse`.
        pub fn contains(&self, other: Self) -> bool {
            if self.len < other.len {
                return false;
            }
            for rot in 0..(self.len - other.len + 1) {
                let a = self.val >> rot;
                let z = 128 - other.len as u32;
                let a = a << z;
                let a = a >> z;
                if a == other.val {
                    return true;
                }
            }
            false
        }

        /// Checks if zeros and ones are even.
        pub fn balanced(&self) -> bool {
            self.val.count_zeros() - (128 - self.len as u32) == self.val.count_ones()
        }

        /// Reverses the bits within the morse code.
        pub fn reversed(self) -> Self {
            let val = self.val << 128 - self.len as u32;
            let val = val.reverse_bits();
            Self { val, len: self.len }
        }
    }

    // impl std::fmt::Display for Point {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "({}, {})", self.x, self.y)
    //     }
    // }

    /// From string with only `'a'..='z'`.
    pub struct AlphaStr<'a>(pub &'a str);
    impl From<AlphaStr<'_>> for Morse {
        fn from(AlphaStr(letters): AlphaStr) -> Self {
            Morse::from(letters.as_bytes())
        }
    }

    /// From string with only `-` and `.`.
    pub struct MorseStr<'a>(pub &'a str);
    impl From<MorseStr<'_>> for Morse {
        fn from(MorseStr(morses): MorseStr) -> Self {
            let mut m = Self::default();
            for morse in morses.bytes() {
                m.push_bit(morse == b'.');
            }
            m
        }
    }

    impl From<&[u8]> for Morse {
        fn from(letters: &[u8]) -> Self {
            let mut m = Self::default();
            for &letter in letters {
                m.push(letter.into());
            }
            m
        }
    }

    impl From<(u8, u8)> for Morse {
        fn from((len, val): (u8, u8)) -> Self {
            let val = val as _;
            Self { val, len }
        }
    }

    impl From<u8> for Morse {
        /// Converts lower case letter range into `Morse` code.
        fn from(letter: u8) -> Self {
            CODES[(letter - b'a') as usize].into()
        }
    }

    pub const CODES: [(u8, u8); 26] = [
        (2, 0b10),   // a
        (4, 0b0111), // b
        (4, 0b0101), // c
        (3, 0b011),  // d
        (1, 0b1),    // e
        (4, 0b1101), // f
        (3, 0b001),  // g
        (4, 0b1111), // h
        (2, 0b11),   // i
        (4, 0b1000), // j
        (3, 0b010),  // k
        (4, 0b1011), // l
        (2, 0b00),   // m
        (2, 0b01),   // n
        (3, 0b000),  // o
        (4, 0b1001), // p
        (4, 0b0010), // q
        (3, 0b101),  // r
        (3, 0b111),  // s
        (1, 0b0),    // t
        (3, 0b110),  // u
        (4, 0b1110), // v
        (3, 0b100),  // w
        (4, 0b0110), // x
        (4, 0b0100), // y
        (4, 0b0011), // z
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_codes() {
        let idx_test = |letter: u8, idx: usize| {
            let letter: Morse = letter.into();
            let code: Morse = CODES[idx].into();
            assert_eq!(letter, code);
        };

        for (idx, letter) in (b'a'..=b'z').enumerate() {
            idx_test(letter, idx);
        }
    }

    #[test]
    fn morse_push() {
        let mut m = Morse::default();

        m.push(b'a'.into());
        assert_eq!(m, Morse { len: 2, val: 0b10 });

        m.push(b'a'.into());
        assert_eq!(
            m,
            Morse {
                len: 2 + 2,
                val: 0b_10_10
            }
        );

        m.push(b'z'.into());
        assert_eq!(
            m,
            Morse {
                len: 2 + 2 + 4,
                val: 0b_10_10_0011
            }
        );

        m.push(b't'.into());
        assert_eq!(
            m,
            Morse {
                len: 2 + 2 + 4 + 1,
                val: 0b_10_10_0011_0
            }
        );

        assert_eq!(
            Morse::from(&b"programmer"[..]),
            Morse {
                len: 26,
                val: 0b_1001_101_000_001_101_10_00_00_1_101
            }
        );
    }

    #[test]
    fn morse_map() {
        use std::collections::HashMap;
        let mut map: HashMap<Morse, u8> = HashMap::default();
        let ambig = ["needing", "nervate", "niding", "tiling"];
        for &x in ambig.iter() {
            *map.entry(AlphaStr(x).into()).or_default() += 1;
        }
        assert_eq!(map.get(&AlphaStr("needing").into()), Some(&4));
    }

    #[test]
    fn within_morses() {
        let cont = |a, b| Morse::from(MorseStr(a)).contains(Morse::from(MorseStr(b)));

        assert!(!cont("--..--", "---"));
        assert!(cont("---..--", "---"));
        assert!(cont(".---..--", "---"));

        assert!(cont("--..--", ".--"));
        assert!(cont("--..--", ".--"));
        assert!(cont("--..--", "--."));
        assert!(cont("-...-", "..."));
        assert!(cont("---------..--------", "-.."));
        assert!(cont("...", ".."));
        assert!(cont("...", "."));
        assert!(cont(".", "."));
        assert!(cont(".----", "."));
        assert!(!cont("-----", "."));
    }

    #[test]
    fn reversing() {
        let rev = |a, b| {
            assert_eq!(
                Morse::from(MorseStr(a)).reversed(),
                Morse::from(MorseStr(b))
            )
        };

        rev(".-", "-.");
        rev("-.-", "-.-");
    }

    #[test]
    fn bonuses() {
        bonus::one();
        bonus::two();
        bonus::four();
        bonus::three();
    }
}
