//! **Tiny & Easy Inputer**
//! A simple wrapping of std::io::{Stdin, BufReader}

use std::io::{self, Stdin, Read, BufRead, BufReader};
use std::str::FromStr;
use std::marker::PhantomData;
use std::iter::Iterator;
use std::convert::From;

/// Tiny and easy inputer
pub struct Tein<R: Read> {
    inner: BufReader<R>,
    buf_line: String,
    idx: usize,
}

impl<R: Read> Tein<R> {
    /// Constructs a new `Tein<R>`
    pub fn new(reader: R) -> Tein<R> {
        Tein {
            inner: BufReader::new(reader),
            buf_line: String::new(),
            idx: 0,
        }
    }
}

/// Constructs a new `Tein<Stdin>`
pub fn new_with_stdin() -> Tein<Stdin> {
    Tein::new(io::stdin())
}

impl<T, R> Input<T, R> for Tein<R>
    where T: FromStr,
          R: Read
{
    fn read(&mut self) -> Option<T> {
        loop {
            if let Some(beg_idx) = self.buf_line[self.idx..].find(|c| !char::is_whitespace(c)) {
                let beg_idx = self.idx + beg_idx;
                let end_idx = beg_idx +
                              match self.buf_line[beg_idx..].find(char::is_whitespace) {
                    Some(n) => n,
                    None => self.buf_line[beg_idx..].len(),
                };
                self.idx = end_idx;
                return self.buf_line[beg_idx..end_idx].parse().ok();
            } else {
                self.buf_line.clear();
                if let Ok(n) = self.inner.read_line(&mut self.buf_line) {
                    if n > 0 {
                        self.idx = 0;
                        continue;
                    }
                }
            }
            return None;
        }
    }

    fn iter(&mut self) -> Iter<T, R> {
        Iter {
            tein: self,
            _type: PhantomData,
        }
    }
}

impl<R: Read> From<R> for Tein<R> {
    fn from(reader: R) -> Self {
        Tein::new(reader)
    }
}

pub trait Input<T, R: Read> {
    /// Read a object from inputer
    ///
    /// # Examples
    /// ```
    /// use tein::*;
    /// let mut teiner = new_with_stdin();
    /// let n: i32 = teiner.read().unwrap_or(0);
    /// ```
    fn read(&mut self) -> Option<T>;
    /// Get the Iter of inputer
    ///
    /// # Examples
    /// ```
    /// use tein::*;
    /// let mut teiner = new_with_stdin();
    /// // read 3 numbers from tein to vector.
    /// let nums: Vec<i32> = teiner.iter().take(3).collect();
    /// ```
    fn iter(&mut self) -> Iter<T, R>;
}


pub struct Iter<'a, T, R: 'a + Read> {
    tein: &'a mut Tein<R>,
    _type: PhantomData<T>,
}

impl<'a, T, R> Iterator for Iter<'a, T, R>
    where T: FromStr,
          R: Read
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.tein.read()
    }
}
