*Rust 1.41 stable*

Simple necklace equality solution; average runtime on dataset is
around \~22.5ns.


    fn is_necklace(a: &str, b: &str) -> bool {
        a.len() == b.len() && [a, a].concat().contains(b)
    }

There's one tiny allocation in `.concat()` which allocates a `String`
required for concatenation, but doing rotation with string slicing
instead is possible, and gives an average runtime of \~17.5ns.

    fn is_necklace(a: &str, b: &str) -> bool {
        let check = |(rotation, _)| {
            let a = (&a[rotation..], &a[..rotation]);
            let b = (&b[..a.0.len()], &b[a.0.len()..]);
            a == b
        };
        a.len() == b.len() && (a.len() == 0 || a.char_indices().any(check))
    }

*Bonus 2*

I decided to try out canonicalization I've seen others do here. All
that is required is to rotate the string and find the maximum
ordering.

At first I tried a solution that involved buffering a bunch of strings
and then comparing them, that was ugly and slow, but eventually I
narrowed down to nothing else then string slincing essentially.

This has a runtime of about ~185ns on my system.

    fn canonicalize(x: &str) -> String {
        x.char_indices()
            .map(|(rotation, _)| [&x[rotation..], &x[..rotation]])
            .max()
            .unwrap_or([x, ""])
            .concat()
    }

Dumping all that into a `HashMap` to find duplicates gives me a
runtime of about \~120ms to find the 4 words. Initializing the
`HashMap` with a known capacity gets it down to \~100ms.

    fn find_the_four<'a>(words: &'a [&'a str]) -> Option<Vec<&'a str>> {
        let mut results = HashMap::with_capacity(words.len());
        for &word in words {
            let result = results.entry(canonicalize(word)).or_insert(Vec::new());
            result.push(word);
            if result.len() == 4 {
                return Some(result.clone());
            }
        }
        None
    }

There's clearly no need for that `String` to be hanging around,
taking advantage of that we could hash it directly from the
slices. 

This has a runtime of \~75ns *(cutting runtime down by only
10ns)* but saves us a lot later down the line, taking the complete
solution `find_the_four` all the way down to \~70ms.

    fn canonicalize_hash(x: &str) -> u64 {
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

Unfortunately now we're just blindly ignoring hash collisions here,
luckily `HashMap` takes a type implementing `Eq` which does that for
us, so we just need to implement a type that defines necklace
equality and hashing.

*Complete Solution*

    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

    struct Necklace<'a>(&'a str);

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


    fn main() {
        let v: Vec<&str> = include_str!("../inputs/enable1.txt")
            .trim()
            .split("\n")
            .collect();
        println!("{:?}", slicer::find_the_four(&v));
    }

----------------------------------

*Solution 2*

I've managed to improve performance of my [previous
solution](https://www.reddit.com/r/dailyprogrammer/comments/ffxabb/20200309_challenge_383_easy_necklace_matching/fkbwpxx/)
after a bit of benchmarking, I've calculated vector allocation as
taking up nearly half of the entire runtime, majority of which where
server no purpose, doing nothing else then holding onto a single
word. 

My optimization was to replace these vectors with counters, such that
I could find 1 solution vastly more efficiently, but obviously then
you lose the other 3 words you'll need, and sorting the vector isn't
actually faster.

The trick is to realize that the `enable1.txt` dataset isn't just a
random word list, it's actually a sorted word list, which makes it
possible to do a binary search, from the rotations of the 1
solution.

This cuts the runtime from ~70ms down to ~42ms.

    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

    fn main() {
        let v: Vec<&str> = include_str!("../inputs/enable1.txt")
            .trim()
            .split("\n")
            .collect();
        println!("{:?}", find_the_four_counters(&v));
    }

    pub fn find_the_four_counters<'a>(words: &'a [&'a str]) -> Option<Vec<&'a str>> {
        // find one solution with hashmap of counters
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

        // find other solutions with binary search of rotations
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

    /// Calculates rotation from canonicalized form.
    pub fn canonicalize_rotation(x: &str) -> usize {
        x.char_indices()
            .map(|(rotation, _)| [&x[rotation..], &x[..rotation]])
            .max()
            .unwrap_or([x, ""])[1]
            .len()
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
