//!
//! Custom string splitter iterator that mimics `str::split()` behavior.
//! Provides an iterator over substrings separated by a given delimiter.

#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/// Custom string splitter iterator.
#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub fn untilchar<'haystack, 'delimiter>(haystack: &'haystack str, delimiter: &'delimiter str) -> &'haystack str{
        let delim = format!("{}", delimiter);
        StrSplit::new(haystack, &delim).next().expect("msg")
}

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item =  &'haystack str;

    /// Advances the iterator and returns the next substring.
    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.remainder.as_mut()?;
        if let Some(delimeter_idx) = remaining.find(self.delimiter){
            let until_delim = &remaining[..delimeter_idx];
            self.remainder = Some(&remaining[delimeter_idx+self.delimiter.len()..]);
            Some(until_delim)
        }else{
            self.remainder.take()
        }

    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
    let letters2: Vec<&str> = haystack.split(" ").collect();
    assert_eq!(letters, letters2);
}
#[test]
fn it_works2() {
    let haystack = "a b c d e ";
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
    let letters2: Vec<&str> = haystack.split(" ").collect();
    assert_eq!(letters, letters2);
}

#[test]
fn it_works3() {
    let haystack = "hello world";
    let letters = untilchar(haystack, "o");
   
    assert_eq!(letters, "hell");
}