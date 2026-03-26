//!
//! Custom string splitter iterator that mimics `str::split()` behavior.
//! Provides an iterator over substrings separated by a given delimiter.

#![warn(rust_2018_idioms)]

/// Custom string splitter iterator.
#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    //D for &str or char cases
    remainder: Option<&'haystack str>,
    delimiter: D,
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
    //return range of delimeter as char can also have range of indices
}

impl Delimiter for &str {
    //self is delimiter and s is haystack
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    //self is delimiter and s is haystack
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices().find(|(_,c)| c==self).map(|(idx,c)| (idx, idx+c.len_utf8()))
    }
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub fn untilchar(
    haystack:&str,
    delim: char,
) -> &'_ str {
    StrSplit::new(haystack, delim).next().expect("msg")
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;

    /// Advances the iterator and returns the next substring.
    fn next(&mut self) -> Option<Self::Item> {
      
        let remaining = self.remainder.as_mut()?;
        if  let Some((start,end))= self.delimiter.find_next(remaining){
            let until_delim = &remaining[..start];
            self.remainder = Some(&remaining[end..]);
            Some(until_delim)
        } else {
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
    let letters = untilchar(haystack, 'o');

    assert_eq!(letters, "hell");
}
