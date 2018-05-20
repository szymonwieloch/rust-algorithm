use std::iter::{Extend, FromIterator};
use std::default::Default;
use std::ops::{Add, AddAssign, Sub};

/**
Quickly calculates sums of consecutive elements in a provided series.

Normally calculating consecutive sums of an ordered collection has O(n) complexity.
Performing k queries has log(n*k) complexity.
But PrefixSum requires only log(n) for the initial calculations and then it can perform
queries in O(1) time. Therefore performing k queries has only log(n+k) complexity.

**More:** <https://en.wikipedia.org/wiki/Prefix_sum>

# Complexity

- Initialization complexity: O(n)
- Query complexity: O(1)
- Memory complexity: O(log(n))

# Example

```
extern crate algorithm;
use algorithm::math::PrefixSum;
use std::iter::FromIterator;

fn main(){
    let arr = [1, 3, 6, 3, 0, 8, 5];
    let cs = PrefixSum::from_iter(&arr);
    let s = cs.between(1, 4); //3+6+3
    assert_eq!(s, 12);
}
```
*/
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PrefixSum<T>
where
    T: Default + Add + Sub + AddAssign<T>,
{
    sums: Vec<T>,
    current_sum: T,
}

impl<T> PrefixSum<T>
where
    T: Default + Clone + Add<Output = T> + Sub<Output = T> + AddAssign<T>,
{
    /**
    Calculates sum of all elements between indices 'from' and 'to'.

    # Complexity

    - Processing complexity: O(1)
    - Memory complexity: O(1)
    */
    pub fn between(&self, from: usize, to: usize) -> T {
        self.sums[to].clone() - self.sums[from].clone()
    }

    ///Creates a new instance of PrefixSum.
    pub fn new() -> Self {
        Self {
            sums: vec![T::default()],
            current_sum: T::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut sums = Vec::with_capacity(capacity +1);
        sums.push(T::default());
        Self{
            sums,
            current_sum: T::default()
        }

    }

    pub fn push(&mut self, val: T){
        self.current_sum += val;
        self.sums.push(self.current_sum.clone());
    }
}

impl<T> Extend<T> for PrefixSum<T>
where
    T: Default + Clone + Add<Output = T> + Sub<Output = T> + AddAssign<T>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.sums.reserve(iter.size_hint().0);
        for i in iter {
            self.push(i);
        }
    }
}

impl<'a, T> Extend<&'a T> for PrefixSum<T>
where
    T: Default + Clone + Add<Output = T> + Sub<Output = T> + AddAssign<T>,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.sums.reserve(iter.size_hint().0);
        for i in iter.into_iter().map(|e| e.clone()) {
            self.push(i);
        }
    }
}

impl<T> FromIterator<T> for PrefixSum<T>
where
    T: Default + Clone + Add<Output = T> + Sub<Output = T> + AddAssign<T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut result = Self::with_capacity(iter.size_hint().0);
        for i in iter.into_iter() {
           result.push(i);
        }
        result
    }
}

impl<'a, T> FromIterator<&'a T> for PrefixSum<T>
where
    T: Default + Clone + Add<Output = T> + Sub<Output = T> + AddAssign<T>,
{
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut result = Self::with_capacity(iter.size_hint().0);
        for i in iter.into_iter().map(|ref e| (*e).clone()) {
           result.push(i);
        }
        result
    }
}

impl<T> Default for PrefixSum<T>
where
    T: Default + Clone + Add<Output = T> + Sub<Output = T> + AddAssign<T>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let arr = [3, 6, 4, 2, 1];
        let ps: PrefixSum<i32> = PrefixSum::from_iter(&arr);
        assert_eq!(ps.between(1,3), 10);
        assert_eq!(ps.between(0,5), 16);
        assert_eq!(ps.between(3,3), 0);
    }

    #[test]
    fn reversed() {
        let arr = [3, 6, 4, 2, 1];
        let ps: PrefixSum<i32> = PrefixSum::from_iter(&arr);
        assert_eq!(ps.between(3, 1), -10);
        assert_eq!(ps.between(5, 0), -16);
        assert_eq!(ps.between(3,3), 0);
    }

    #[should_panic]
    #[test]
    fn too_big_indexes() {
        let arr = [3, 6, 4, 2, 1];
        let ps: PrefixSum<i32> = PrefixSum::from_iter(&arr);
        ps.between(2, 7);
    }
}
