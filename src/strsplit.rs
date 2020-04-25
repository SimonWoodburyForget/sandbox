//! String splitting.

pub trait FnOption<'a>: FnMut() -> Option<&'a str> {}
impl<'a, T> FnOption<'a> for T where T: FnMut() -> Option<&'a str> {}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

pub fn split(haystack: &str, delimiter: impl Delimiter) -> impl FnOption {
    let mut remainder = Some(haystack);
    move || {
        let r = remainder.as_mut()?;
        if let Some((start, end)) = delimiter.find_next(r) {
            let until_delimiter = &r[..start];
            *r = &r[end..];
            Some(until_delimiter)
        } else {
            remainder.take()
        }
    }
}

pub fn split_ws(haystack: &str) -> impl FnOption {
    split(haystack, " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let mut letters = split(haystack, " ");
        assert_eq!(letters(), Some("a"));
        assert_eq!(letters(), Some("b"));
        assert_eq!(letters(), Some("c"));
        assert_eq!(letters(), Some("d"));
        assert_eq!(letters(), Some("e"));
        assert_eq!(letters(), None);

        let split_ws = |input| std::iter::from_fn(split(input, " "));
        let letters: Vec<_> = split_ws(haystack).collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tail() {
        let haystack = "a b c ";
        let letters = std::iter::from_fn(split(haystack, " "));
        let v: Vec<_> = letters.collect();
        assert_eq!(v, vec!["a", "b", "c", ""]);
    }
}
