//!
//! Custom string splitter iterator that mimics `str::split()` behavior.
//! Provides an iterator over substrings separated by a given delimiter.

#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/// Custom string splitter iterator.
#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item =  &'a str;

    /// Advances the iterator and returns the next substring.
    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.remainder.as_mut()?;
        if let Some(delimeter_idx) = remaining.find(" "){
            let until_delim:&'a str = &remaining[..delimeter_idx];
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