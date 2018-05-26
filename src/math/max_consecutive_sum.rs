use std::iter::IntoIterator;
use std::ops::Add;

/**
Calculates maximum sum of consecutive elements in the provided collection.

This is an implementation of the Kadane's algorithm.
If no positive consecutive sum is found, function returns T::default()( usually zero).

**More:** <https://en.wikipedia.org/wiki/Maximum_subarray_problem>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

# Example

```
extern crate algorithm;
use algorithm::math::max_consecutive_sum::max_consecutive_sum;

fn main() {
    let arr = [2, -5, 6, 8, 3, -9, 2, 3];
    let iter = arr.iter().map(|&e|e);
    //max consecutive sum is 6+8+3 = 17
    assert_eq!(max_consecutive_sum(iter), 17);
}
```
*/
pub fn max_consecutive_sum<I, T>(iter: I) -> T
where
    I: IntoIterator<Item = T>,
    T: Default + Ord + Clone + Add<Output = T>,
{
    let mut curr = T::default();
    let mut max_sum = T::default();
    for i in iter.into_iter() {
        curr = (curr + i).max(T::default());
        max_sum = max_sum.max(curr.clone())
    }
    max_sum
}

/**
Calculates maximum sum of consecutive elements in the provided collection.

This is an implementation of the Kadane's algorithm.
Additionally returns start and end indexes of the slice that holds the greatest sum.
If there is more than one slice with the same greatest sum, indexes of the one with the lowest
start index are returned.
If no positive consecutive sum is found, function returns (T::default(), 0, 0) ).

**More:** <https://en.wikipedia.org/wiki/Maximum_subarray_problem>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

# Example

```
extern crate algorithm;
use algorithm::math::max_consecutive_sum::max_consecutive_sum_idx;

fn main() {
    let arr = [2, -5, 6, 8, 3, -9, 2, 3];
    let iter = arr.iter().map(|&e|e);
    //max consecutive sum is 6+8+3 = 17
    let (mcs, start, end) = max_consecutive_sum_idx(iter);
    assert_eq!(mcs, 17);
    assert_eq!(start, 2);
    assert_eq!(end, 5);
}
```
*/
pub fn max_consecutive_sum_idx<I, T>(iter: I) -> (T, usize, usize)
where
    I: IntoIterator<Item = T>,
    T: Default + Ord + Clone + Add<Output = T>,
{
    let mut curr = T::default();
    let mut begin: usize = 0;
    let mut start_idx: usize = 0;
    let mut end_idx: usize = 0;
    let mut max_sum = T::default();
    for (curr_idx, val) in iter.into_iter().enumerate() {
        curr = curr + val;
        if curr < T::default() {
            curr = T::default();
            begin = curr_idx + 1;
        }
        if curr > max_sum {
            max_sum = curr.clone();
            start_idx = begin;
            end_idx = curr_idx + 1;
        }
    }
    (max_sum, start_idx, end_idx)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let arr: [i32; 0] = [];
        assert_eq!(max_consecutive_sum(arr.iter().map(|&e|e)), 0);
    }

    #[test]
    fn single() {
        let arr = [6];
        assert_eq!(max_consecutive_sum(arr.iter().map(|&e|e)), 6);
    }

    #[test]
    fn negative() {
        let arr = [-4 -6, -2];
        assert_eq!(max_consecutive_sum(arr.iter().map(|&e|e)), 0);
    }

    #[test]
    fn simple() {
        let arr = [-4, 4, 5, 2,  -7, 6, -2];
        assert_eq!(max_consecutive_sum(arr.iter().map(|&e|e)), 11);
    }

    #[test]
    fn idx_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(max_consecutive_sum_idx(arr.iter().map(|&e|e)), (0, 0, 0));
    }

    #[test]
    fn idx_single() {
        let arr = [6];
        assert_eq!(max_consecutive_sum_idx(arr.iter().map(|&e|e)), (6, 0, 1));
    }

    #[test]
    fn idx_negative() {
        let arr = [-4 -6, -2];
        assert_eq!(max_consecutive_sum_idx(arr.iter().map(|&e|e)), (0, 0, 0));
    }

    #[test]
    fn idx_simple() {
        let arr = [-4, 4, 5, 2,  -7, 6, -2];
        assert_eq!(max_consecutive_sum_idx(arr.iter().map(|&e|e)), (11, 1, 4));
    }

}