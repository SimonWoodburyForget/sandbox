//! Daily programming challenge 380.

const ENABLE_1: &str = include_str!("enable1.txt");

pub mod bonus {
    use super::*;

    pub fn one() -> Morse<Code> {
        use std::collections::HashMap;
        let mut map: HashMap<Morse<Code>, u8> = HashMap::default();
        let mut result = None;
        let words = ENABLE_1.trim().split("\n");
        for x in words {
            let m: Morse<Code> = AlphaStr(x).into();
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

    pub fn two() -> Morse<Code> {
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

    pub fn four() -> Morse<Code> {
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
    use std::ops::*;

    /// Default bit holder, `u128` was picked because there were
    /// a few very long bit sequences.
    pub type Code = u128;

    pub trait Codes:
        BitOrAssign
        + ShlAssign<u8>
        + From<bool>
        + Shl<u8, Output = Self>
        + Shr<u8, Output = Self>
        + Eq
        + Copy
        + Sized
    {
        fn count_zeros(self) -> u32;
        fn count_ones(self) -> u32;
        fn reverse_bits(self) -> Self;
    }

    macro_rules! impl_codes {
        ( $($ty:ty),* ) => {
            $(
                impl Codes for $ty {
                    fn count_zeros(self) -> u32 {
                        self.count_zeros()
                    }

                    fn count_ones(self) -> u32 {
                        self.count_ones()
                    }

                    fn reverse_bits(self) -> Self {
                        self.reverse_bits()
                    }
                }
            )*
        };
    }

    impl_codes!(u128, u64, u32, u16, u8);

    const fn max_len<T: Sized>() -> u8 {
        std::mem::size_of::<T>() as u8 * 8
    }

    /// Bit encoded morse code, each dot-or-dash is encoded as a 1 (dot) or 0 (dash),
    /// with the lenght of said sequence, which is done to make it as small as possible.
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct Morse<T: Codes> {
        /// Length of included bits in value.
        pub len: u8,

        /// Morse code bits (u128, u64, u32, ...)
        pub val: T,
    }

    impl<T: Codes> Morse<T> {
        /// Push another morse on top of this one, like a letter.
        pub fn push(&mut self, code: Self) {
            self.len += code.len;
            self.val <<= code.len;
            self.val |= code.val;
        }

        /// Push one bit onto the Morse code.
        pub fn push_bit(&mut self, bit: bool) {
            self.push(Self {
                len: 1,
                val: bit.into(),
            })
        }
    }

    impl<T: Codes> Morse<T> {
        /// Checks if `Morse` contains another `Morse`.
        pub fn contains(&self, other: Self) -> bool {
            if self.len < other.len {
                return false;
            }
            for rot in 0..(self.len - other.len + 1) {
                let a = self.val >> rot;
                let z = max_len::<T>() - other.len;
                let a = a << z;
                let a = a >> z;
                if a == other.val {
                    return true;
                }
            }
            false
        }
    }

    impl<T: Codes> Morse<T> {
        /// Checks if zeros and ones are even.
        pub fn balanced(&self) -> bool {
            self.val.count_zeros() - (max_len::<Code>() as u32 - self.len as u32)
                == self.val.count_ones()
        }

        /// Reverses the bits within the morse code.
        pub fn reversed(self) -> Self {
            let val = self.val << max_len::<Code>() - self.len;
            let val = val.reverse_bits();
            Self { val, len: self.len }
        }
    }

    impl std::fmt::Display for Morse<Code> {
        /// Displaying mores code is done with bit shifting, keep in mind that a binary number
        /// such as `0b1110` would actually produce sequence `"-..."`.
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // NOTE: `b' '` could really be anything else.
            let mut buffer = [b' '; max_len::<Code>() as usize];
            for x in 0..self.len {
                let one = self.val >> (self.len - 1 - x) & 1;
                buffer[x as usize] = if one == 1 { b'.' } else { b'-' };
            }
            write!(f, "{}", unsafe {
                // SAFETY: Safe because we only encode valid utf8 characters.
                std::str::from_utf8_unchecked(&buffer[0..self.len as usize])
            })
        }
    }

    /// From string with only `'a'..='z'`.
    pub struct AlphaStr<'a>(pub &'a str);
    impl From<AlphaStr<'_>> for Morse<Code> {
        fn from(AlphaStr(letters): AlphaStr) -> Self {
            Morse::from(letters.as_bytes())
        }
    }

    /// From string with only `-` and `.`.
    pub struct MorseStr<'a>(pub &'a str);
    impl From<MorseStr<'_>> for Morse<Code> {
        fn from(MorseStr(morses): MorseStr) -> Self {
            let mut m = Self::default();
            for morse in morses.bytes() {
                m.push_bit(morse == b'.');
            }
            m
        }
    }

    impl From<&[u8]> for Morse<Code> {
        fn from(letters: &[u8]) -> Self {
            let mut m = Self::default();
            for &letter in letters {
                m.push(letter.into());
            }
            m
        }
    }

    impl From<(u8, u8)> for Morse<Code> {
        fn from((len, val): (u8, u8)) -> Self {
            let val = val as _;
            Self { val, len }
        }
    }

    impl From<u8> for Morse<Code> {
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

/// Morse code encoding generation.
pub mod codes {
    use super::{Code, Morse};

    const MAX: usize = u8::max_value() as usize;

    /// Morse code iterator. Iterating morse code is a two step process, you need to go
    /// through all possible binary representation in all possible lenghts.
    ///
    /// NOTE: sequence order may not be exactly what you'd expect as a result of `--. == 100`
    pub fn iter(Morse { mut val, mut len }: Morse<Code>) -> impl Iterator<Item = Morse<Code>> {
        let mut result = None;
        std::iter::from_fn(move || {
            result = Some(Morse { val, len });
            if val < (1 << len) - 1 {
                val += 1;
            } else {
                len += 1;
                val = 0;
            }
            result
        })
    }

    /// Iterates a range of morse codes, where `start` is inclusive and `end` is exclusive.
    pub fn range(start: Morse<Code>, end: Morse<Code>) -> impl Iterator<Item = Morse<Code>> {
        iter(start).take_while(move |&x| x != end)
    }

    use counter::Counter;
    mod counter {
        #[derive(Default, Copy, Clone)]
        pub struct Counter<T> {
            counter: usize,
            pub value: T,
        }

        impl<T> Counter<T> {
            pub fn incr(&mut self) {
                self.counter += 1;
            }
        }

        impl<T> Eq for Counter<T> {}
        impl<T> PartialEq for Counter<T> {
            fn eq(&self, other: &Self) -> bool {
                self.counter == other.counter
            }
        }

        impl<T> Ord for Counter<T> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                other.counter.cmp(&self.counter)
            }
        }

        impl<T> PartialOrd for Counter<T> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                other.counter.partial_cmp(&self.counter)
            }
        }
    }

    /// Generates a byte mapping of morse codes for the input, where the most common
    /// byte is mapped to the smallest morse code value.
    pub fn gen(input: &[u8]) -> [Morse<Code>; MAX] {
        let mut counters = [Counter::<usize>::default(); MAX];
        for (e, counter) in counters.iter_mut().enumerate() {
            counter.value = e;
        }

        for &byte in input {
            counters[byte as usize].incr();
        }

        counters.sort();
        let mut mapping = [Morse::<Code>::default(); MAX];
        for (code, counter) in iter(Morse::default())
            .take(MAX)
            .skip(1)
            .zip(counters.iter())
        {
            mapping[counter.value] = code;
        }

        mapping
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_codes() {
        let idx_test = |letter: u8, idx: usize| {
            let letter: Morse<Code> = letter.into();
            let code: Morse<Code> = CODES[idx].into();
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
        let mut map: HashMap<Morse<Code>, u8> = HashMap::default();
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
    fn encoding() {
        assert_eq!(Morse { len: 1, val: 0b1 }.to_string(), ".");
        assert_eq!(Morse { len: 2, val: 0b01 }.to_string(), "-.");
        assert_eq!(Morse { len: 2, val: 0b11 }.to_string(), "..");
        assert_eq!(Morse { len: 2, val: 0b01 }.to_string(), "-.");
        assert_eq!(Morse { len: 3, val: 0b101 }.to_string(), ".-.");
        assert_eq!(Morse { len: 3, val: 0b010 }.to_string(), "-.-");
        assert_eq!(Morse::from(MorseStr(".")).to_string(), ".");
        assert_eq!(Morse::from(MorseStr(".-.-")).to_string(), ".-.-");
        assert_eq!(Morse::from(MorseStr("----")).to_string(), "----");
        assert_eq!(Morse::from(MorseStr("....")).to_string(), "....");
        assert_eq!(Morse::from(AlphaStr("a")).to_string(), ".-");
        assert_eq!(
            Morse::from(AlphaStr("programmer")).to_string(),
            ".--..-.-----..-..-----..-."
        );
    }

    #[test]
    fn morse_iter() {
        assert_eq!(
            codes::iter(Default::default())
                .take(17)
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            vec![
                "", "-", ".", "--", "-.", ".-", "..", "---", "--.", "-.-", "-..", ".--", ".-.",
                "..-", "...", "----", "---."
            ]
        );
        assert_eq!(
            codes::range(MorseStr("---").into(), MorseStr("----").into())
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            vec!["---", "--.", "-.-", "-..", ".--", ".-.", "..-", "..."]
        );
        let mut it =
            codes::range(Morse { len: 5, val: 0 }, Morse { len: 6, val: 0 }).map(|x| x.to_string());
        assert_eq!(it.next().unwrap(), "-----");
        assert_eq!(it.last().unwrap(), ".....");
    }

    #[test]
    fn morse_gen() {
        let mapping = codes::gen(b"aabc");
        assert_eq!(mapping[b'a' as usize], Morse { len: 1, val: 0 });

        let mapping = codes::gen(b"abcc");
        assert_eq!(mapping[b'c' as usize], Morse { len: 1, val: 0 });
    }

    #[test]
    fn bonuses() {
        bonus::one();
        bonus::two();
        bonus::four();
        bonus::three();
    }
}
