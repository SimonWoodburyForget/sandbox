*Rust 1.41 stable*

Simple necklace equality solution; average runtime on dataset is
around ~22.5ns.


    fn is_necklace(a: &str, b: &str) -> bool {
        a.len() == b.len() && [a, a].concat().contains(b)
    }

There's one tiny allocation in `.concat()` which allocates a `String`
required for string concatenation; doing rotation with string slicing
instead averages a runtime of ~17.5ns.


    fn is_necklace(a: &str, b: &str) -> bool {
        let check = |(rotation, _)| {
            let a = (&a[rotation..], &a[..rotation]);
            let b = (&b[..a.0.len()], &b[a.0.len()..]);
            a == b
        };
    
        a.len() == b.len() && (a.len() == 0 || a.char_indices().any(check))
    }

*Bonus 2*

Decided to try out canonicalization I've seen others do here; a simple
slice rotation to find the  maximum ordering, followed by dumping the
results into a string results in a runtime of about
~185ns. *(concatenating all slices to compare strings instead would
result in a runtime of ~500ns, buffering into another vector to
compare later would result in a runtime of ~1.2us)*


    fn canonicalize(x: &str) -> String {
        x.char_indices()
            .map(|(rotation, _)| [&x[rotation..], &x[..rotation]])
            .max()
            .unwrap_or([x, ""])
            .concat()
    }

Dumping all that into a hashmap to find duplicates gives me a runtime
of about ~120ms. *(a similar Python implementation takes ~990ms)*


    fn find_the_four<'a>(words: &'a [&'a str]) -> Option<Vec<&'a str>> {
        let mut results: HashMap<String, Vec<&str>> = HashMap::new();
        for word in words {
            let result = results.entry(canonicalize(word)).or_insert(Vec::new());
            result.push(word);
            if result.len() == 4 {
                return Some(result.clone());
            }
        }
        None
    }
