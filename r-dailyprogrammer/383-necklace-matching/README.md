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
