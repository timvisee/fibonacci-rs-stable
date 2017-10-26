#![feature(test)]

extern crate num;

#[cfg(test)]
mod test;

use num::{Zero, One, CheckedAdd};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Fibonacci<T> {
  last_two: (T, T)
}

impl<T> Default for Fibonacci<T>
  where T: Debug + Zero + One
{
  fn default() -> Self {
    Fibonacci { last_two: (T::zero(), T::one()) }
  }
}

impl<T> Iterator for Fibonacci<T>
  where T: Debug + CheckedAdd + Clone
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    let n = self.last_two.0.checked_add(&self.last_two.1);
    if let Some(ref x) = n {
      std::mem::swap(&mut self.last_two.0, &mut self.last_two.1);
      self.last_two.1 = x.clone();
    }
    n
  }
}
