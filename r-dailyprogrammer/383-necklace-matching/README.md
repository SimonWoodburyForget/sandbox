*Rust 1.41 stable* 

Simple necklace equality solution, with an average runtime on the
`enable1` dataset is around ~22.5ns. (with explicit inlining)

    fn is_necklace(a: &str, b: &str) -> bool {
        a.len() == b.len() && [a, a].concat().contains(b)
    }

This is so fast it's likely not worth optimizing, so I decided to go
ahead and optimize it. There's one tiny allocation in `.concat()`
which allocates a `String` which is required for actual string
concatition. This can be replaced with a little string slicing, which
gets us down to around ~17.5ns.

    fn is_necklace(a: &str, b: &str) -> bool {
        let check = |(rotation, _)| {
            let a = (&a[rotation..], &a[..rotation]);
            let b = (&b[..a.0.len()], &b[a.0.len()..]);
            a == b
        };

        a.len() == b.len() && (a.len() == 0 || a.char_indices().any(check))
    }

-----------------

Bonus 2 -- implementing the normalization, which simply means rotating
the string until it reaches a minimum or maximum value of some kind.

Now that it's self evident that we can just slice to rotate, we can
just do that in a loop and find the minimum. The following
implementation has a runtime of about 155ns.

    pub fn canonicalize(x: &str) -> [&str; 2] {
        x.char_indices()
            .map(|(rotation, _)| [&x[rotation..], &x[..rotation]])
            .max()
            .unwrap_or([x, ""])
    }
    
It's interesting to note that you don't need any heap allocations at
all here, not even for the returning slices.


