*Rust 1.41 stable* 

Simple necklace equality solution, with an average runtime on the
`enable1` dataset is around ~22.5ns. (with explicit inlining)

    fn is_necklace(a: &str, b: &str) -> bool {
        a.len() == b.len() && [a, a].concat().contains(b)
    }

This is likely not worth optimizing, so I decided to go ahead and try
to see if it could be optimized at all, and if so by how much.

There's one tiny allocation in `.concat()` which allocates a `String`
allowing the string to be concatitated; I got rid of that with a
little string slicing, which got me down to around ~16.5ns.

    fn is_necklace(a: &str, b: &str) -> bool {
        let check = |rotation| {
            let a = (&a[rotation..], &a[..rotation]);
            let b = (&b[..a.0.len()], &b[a.0.len()..]);
            a == b
        };

        let len = a.len();
        len == b.len() && ((0..len).any(check) || len == 0)
    }
