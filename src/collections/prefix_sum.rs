use std::iter::{FromIterator, Extend};
use std::default::Default;
use std::ops::{Sub, Add, AddAssign};


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
use algorithm::collections::PrefixSum;
use std::iter::FromIterator;

fn main(){
    let arr = [1, 3, 6, 3, 0, 8, 5];
    let cs = PrefixSum::from_iter(arr.iter());
    let s = cs.between(1, 4); //3+6+3
    assert_eq!(s, 12);
}
```
*/
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PrefixSum<T> where T: Default + Add + Sub + AddAssign<T> {
    sums: Vec<T>,
    current_sum: T
}

impl<T> PrefixSum<T> where  T: Default + Clone + Add<Output=T> + Sub<Output=T> + AddAssign<T> {
    /**
    Calculates sum of all elements between indices 'from' and 'to'.

    # Complexity

    - Processing complexity: O(1)
    - Memory complexity: O(1)
    */
    pub fn between(&self, from: usize, to: usize) -> T {
        if from > to {
            panic!("In a range the start value cannot be lower than the end.");
        }
        self.sums[to].clone() - self.sums[from].clone()
    }

    ///Creates a new instance of PrefixSum.
    pub fn new() -> Self {
        Self{
            sums: vec![T::default()],
            current_sum: T::default()
        }
    }
}

impl<T> Extend<T> for PrefixSum<T> where  T: Default + Clone + Add<Output=T> + Sub<Output=T> + AddAssign<T> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        for i in iter{
            self.current_sum += i;
            self.sums.push(self.current_sum.clone())
        }
    }
}

impl<'a, T> Extend<&'a T> for PrefixSum<T> where  T: Default + Clone + Add<Output=T> + Sub<Output=T> + AddAssign<T> {
    fn extend<I: IntoIterator<Item=&'a T>>(&mut self, iter: I) {
        for i in iter.into_iter().map(|e|e.clone()) {
            self.current_sum += i;
            self.sums.push(self.current_sum.clone());
        }
    }
}

impl<T> FromIterator<T> for PrefixSum<T> where  T: Default + Clone + Add<Output=T> + Sub<Output=T> + AddAssign<T>  {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut current_sum: T = T::default();
        let mut sums : Vec<T> = vec![T::default()];
        for i in iter.into_iter(){
            current_sum += i;
            sums.push(current_sum.clone());
        }
        Self{
            current_sum, sums
        }
    }
}

impl<'a, T> FromIterator<&'a T> for PrefixSum<T> where  T: Default + Clone + Add<Output=T> + Sub<Output=T> + AddAssign<T> {
    fn from_iter<I: IntoIterator<Item=&'a T>>(iter: I) -> Self {
        let mut current_sum: T = T::default();
        let mut sums : Vec<T> = vec![T::default()];
        for i in iter.into_iter().map(|ref e|(*e).clone()){
            current_sum += i;
            sums.push(current_sum.clone());
        }
        Self{
            current_sum, sums
        }
    }
}

impl<T> Default for PrefixSum<T> where  T: Default + Clone + Add<Output=T> + Sub<Output=T> + AddAssign<T> {
    fn default() -> Self {
        Self::new()
    }
}

