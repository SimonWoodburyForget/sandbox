I went through the problem myself, and from what I can see you're
overusing strings re-allocations, while a lot of the challenges can
actually be solved inplace by the looks of things. 


If you pay close attention, you'll also note the challenges can be
optimized by just representing the morse-code as bits and bit
shifting/xor/.. operations.


Unsurprisingly morse-code is a highly compressible encoding, a single
character can be compressed down to one bit (`.` or `-` can be `1` or
`0`), by using `u8` for each `.` you're using up to 8 times more
memory, because there's 8-bits in 1-byte, which is going to cost you
heavily once you decide to hash it.


The base of my solutions look something like this:

    /// Bit encoded morse code, each dot-or-dash is encoded as a 1 (dot) or 0 (dash),
    /// with the length of said sequence, which is done to make it as small as possible.
    #[derive(Debug, PartialEq, Default, Hash, Copy, Clone)]
    pub struct Morse {
        len: u8,
        val: u128,
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
    
        pub fn push_bit(&mut self, bit: bool) {
            self.unchecked_push(Morse {
                len: 1,
                val: bit as _,
            })
        }
    }
    
    /// From string with only `'a'..='z'`.
    struct AlphaStr<'a>(&'a str);
    impl From<AlphaStr<'_>> for Morse {
        fn from(AlphaStr(letters): AlphaStr) -> Self {
            Morse::from(letters.as_bytes())
        }
    }
    
    /// From string with only `-` and `.`.
    struct MorseStr<'a>(&'a str);
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

There are potentially other ways of going through with it, mainly the parts I optimized was using bit shifting yes, and removing the `HashMap` look buffer with an array of `[(u8, u8), 26]` because we really don't need a hash map. -- We have the best we could hope for here, because range `'a'..='z'` is contiguous and fixed, so we can just subtract by the start of the range, which gives use a perfectly valid index into an array.

I'll just ignore formatting `0b1010` into `".-.-"` because we actually never need to do that to solve any problem beyond displaying morse-code to the user as utf8.

First bonus is just to find the more code sequence with 13 collisions, so we literally only need counters, which are vastly cheaper then `Vec<String>` like you where doing.

    pub fn bonus_one() -> Morse {
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

*runtime of 45ms on my system.* 

Disclaimer: *my system is a 12 years old machine*

----------

What about checking whether some bits are contained in some other
bits? Again nothing a little bit shifting can't solve. *This took me
longer then it should have...* but I believe the following should be
mostly correct:


    impl Morse {
        /// Checks if `Morse` contains another `Morse`.
        pub fn contains(&self, other: Morse) -> bool {
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
    }

    pub fn bonus_two() -> Morse {
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

*runtime of 16ms on my system.*

------

Counting zeros and ones? Well that's also bit manipulation. `u128` has
some `count_zeros` and `count_ones` utility functions:


    impl Morse {
        pub fn balanced(&self) -> bool {
            self.val.count_zeros() - (128 - self.len as u32) == self.val.count_ones()
        }
    }
    
    pub fn bonus_three() -> Vec<&'static str> {
        let words = ENABLE_1.trim().split("\n");
        let mut results = Vec::new();
        for w in words {
            if w.len() == 21 {
                let x = Morse::from(AlphaStr(w));
                if x.balanced() {
                    results.push(w);
                    if results.len() == 2 {
                        break
                    }
                }
            }
        }
        assert_eq!(results.len(), 2);
        results
    }

*runtime of 3ms on my system.*

-----

Not much harder is reversing bits. We can just shift the bits to the
other end, and then reverse the whole thing.


    impl Morse {
        pub fn reversed(self) -> Self {
            let val = self.val << 128 - self.len as u32;
            let val = val.reverse_bits();
            Self { val, len: self.len }
        }
    }
    
    pub fn bonus_four() -> Morse {
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

*runtime of 2ms on my system.*

-----------------

Bit manipulations, definitely work pretty well here. --The [complete
source](https://gist.github.com/SimonWoodburyForget/1b0bf16450e88c1be4fccd24ceedc523)
so far only comes up to ~200 LoC (without tests or benches). There's
probably *a lot* left to optimize here; I'm just outlining some rough
ideas of how to solve the challenge more efficiently.


All-in-all, to write faster Rust code, you have to pay attention to
your string allocations, unicode operations, byte operations (and bit
operations in this case), knowing when to leverage one over
others. String allocations are definitely very expensive, and so are
vector allocations, while unicode operations are more expensive then
byte operations. Hashing is also great when it's required, but if you
can replace it completely with indexing, then do that instead, and if
you need to hash something make sure it's as small as it can possibly
be before it needs to get dumped into a hashmap. You really also
shouldn't be dumping vectors and strings into hashmaps unless you
actually need the hashmap to own them.

