*Rust 1.41 stable*

Simple necklace equality solution; average runtime on dataset is around \~22.5ns.

    fn is_necklace(a: &str, b: &str) -> bool {
        a.len() == b.len() && [a, a].concat().contains(b)
    }

There's one tiny allocation in `.concat()` which allocates a `String` required for string concatenation; doing rotation with string slicing instead is possible, and averages a runtime of \~17.5ns.

    fn is_necklace(a: &str, b: &str) -> bool {
        let check = |(rotation, _)| {
            let a = (&a[rotation..], &a[..rotation]);
            let b = (&b[..a.0.len()], &b[a.0.len()..]);
            a == b
        };
    
        a.len() == b.len() && (a.len() == 0 || a.char_indices().any(check))
    }

*Bonus 2*

Decided to try out canonicalization I've seen others do here; a simple slice rotation to find the  maximum ordering, followed by dumping the results into a string results in a runtime of about \~185ns. *(concatenating all slices to compare strings instead would result in a runtime of \~500ns, buffering into another vector to compare later would result in a runtime of \~1.2us)*

    fn canonicalize(x: &str) -> String {
        x.char_indices()
            .map(|(rotation, _)| [&x[rotation..], &x[..rotation]])
            .max()
            .unwrap_or([x, ""])
            .concat()
    }

Dumping all that into a `HashMap` to find duplicates gives me a runtime of about \~120ms. Initializing the `HashMap` with a known capacity gets it down to \~100ms. *(for reference, a similar Python implementation takes \~990ms on my system)*

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

Now there's clearly no need for that `String` to be hanging around, taking advantage of that we could hash it directly from the slices. This has a runtime of \~75ns *(cutting runtime down by only 10ns)* but saves us a lot later down the line, taking the complete solution all the way down to \~70ms.

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
