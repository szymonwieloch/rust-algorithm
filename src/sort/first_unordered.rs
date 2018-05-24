use std::iter::{IntoIterator, Iterator};
use sort::Order;
use std::convert::From;
use utils::PairIterator;

/**
Finds index of the first element that is not ordered using a provided comparator.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::first_unordered_by;

fn main() {
    let arr_desc = [3, 5, 7, 7, 7, 4];
    let arr_sorted = [1, 4, 7, 8, 9];
    assert_eq!(first_unordered_by(&arr_desc, |&a, &b| a>b), Some(5));
    assert_eq!(first_unordered_by(&arr_sorted, |&a, &b| a>b), None);
}
```
*/
pub fn first_unordered_by<I, T, F>(iter: I, mut is_unordered: F) -> Option<usize>
where
    I: IntoIterator<Item = T>,
    F: FnMut(T, T) -> bool,
    T: Copy,
{
    let pos = PairIterator::from(iter).position(|(a, b)| is_unordered(a, b));
    //this returns index of pair, but it is the second element that is unsorted
    match pos {
        Some(p) => Some(p+1),
        None => None
    }
}

/**
Finds index of the first element that is not ordered.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::first_unordered;
use algorithm::sort::Order::*;

fn main() {
    let arr_decr = [5, 6, 7, 8, 9, 0];
    assert_eq!(first_unordered(&arr_decr, Increasing), Some(5));
    let arr_incr = [1, 4, 7, 8, 9];
    assert_eq!(first_unordered(&arr_incr, Increasing), None);
}
```
*/
pub fn first_unordered<I, T>(iter: I, order: Order) -> Option<usize>
where
    I: IntoIterator<Item = T>,
    T: PartialOrd,
    T: Copy,
{
    let mut iter = PairIterator::from(iter);
    let pos = match order {
        Order::Increasing => iter.position(|(a, b)| a>=b),
        Order::Decreasing => iter.position(|(a, b)| a<=b),
        Order::NotIncreasing => iter.position(|(a, b)| a<b),
        Order::NotDecreasing => iter.position(|(a, b)| a>b),
    };
    //this returns index of pair, but it is the second element that is unsorted
    match pos {
        Some(p) => Some(p+1),
        None => None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incr_empty() {
        let arr: [i32; 0] = [];
        assert!(first_unordered(&arr, Order::Increasing).is_none());
    }

    #[test]
    fn incr_single() {
        let arr = [5];
        assert!(first_unordered(&arr, Order::Increasing).is_none());
    }

    #[test]
    fn incr_found() {
        let arr = [5, 7, 8, 10, 10, 11, 12];
        assert_eq!(first_unordered(&arr, Order::Increasing), Some(4));
    }

    #[test]
    fn incr_not_found() {
        let arr = [5, 7, 8, 10, 11, 12, 13];
        assert_eq!(first_unordered(&arr, Order::Increasing), None);
    }

    #[test]
    fn decr_empty() {
        let arr: [i32; 0] = [];
        assert!(first_unordered(&arr, Order::Decreasing).is_none());
    }

    #[test]
    fn decr_single() {
        let arr = [5];
        assert!(first_unordered(&arr, Order::Decreasing).is_none());
    }

    #[test]
    fn decr_found() {
        let arr = [44, 33, 22, 22, 11];
        assert_eq!(first_unordered(&arr, Order::Decreasing), Some(3));
    }

    #[test]
    fn decr_not_found() {
        let arr = [44, 33, 22, 15, 11];
        assert_eq!(first_unordered(&arr, Order::Decreasing), None);
    }

    #[test]
    fn ndecr_empty() {
        let arr: [i32; 0] = [];
        assert!(first_unordered(arr.iter(), Order::NotDecreasing).is_none());
    }

    #[test]
    fn ndecr_single() {
        let arr = [5];
        assert!(first_unordered(&arr, Order::NotDecreasing).is_none());
    }

    #[test]
    fn ndecr_found() {
        let arr = [5, 7, 7, 10, 8, 11, 12];
        assert_eq!(first_unordered(&arr, Order::NotDecreasing), Some(4));
    }

    #[test]
    fn ndecr_not_found() {
        let arr = [5, 7, 8, 10, 10, 11, 12];
        assert_eq!(first_unordered(&arr, Order::NotDecreasing), None);
    }

    #[test]
    fn nincr_empty() {
        let arr: [i32; 0] = [];
        assert!(first_unordered(&arr, Order::NotIncreasing).is_none());
    }

    #[test]
    fn nincr_single() {
        let arr = [5];
        assert!(first_unordered(&arr, Order::NotIncreasing).is_none());
    }

    #[test]
    fn nincr_found() {
        let arr = [44, 33, 22, 23, 11];
        assert_eq!(first_unordered(&arr, Order::NotIncreasing), Some(3));
    }

    #[test]
    fn nincr_not_found() {
        let arr = [44, 33, 22, 22, 11];
        assert_eq!(first_unordered(&arr, Order::NotIncreasing), None);
    }
}
