use std::iter::{IntoIterator, Iterator};
use sort::Order;
use utils::PairIterator;
use std::convert::From;

/**
Checks if the provided collection is sorted using provided comparator.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::is_ordered_by;

fn main() {
    let arr_sorted = [3.0,5.0,7.0,9.0];
    let arr_random = [1.0, 7.0, 6.0, 3.0, 0.0];
    assert!(is_ordered_by(&arr_sorted, |&a, &b| b>a));
    assert!(!is_ordered_by(&arr_random, |&a, &b| b>a));
}
```
*/
pub fn is_ordered_by<I, T, F>(iter: I, mut is_ordered: F) -> bool
where
    I: IntoIterator<Item = T>,
    T: Copy,
    F: FnMut(T, T) -> bool,
{
    let mut iter = PairIterator::from(iter.into_iter());
    iter.all(|(a,b)| is_ordered(a, b))
}

/**
Checks if the provided collection is sorted in ascending order.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::is_ordered;
use algorithm::sort::Order::*;

fn main() {
    let incr =      [1,3,5,7,9];
    let not_decr =  [1,7,7,7,9];
    let decr =      [9,8,7,5,3];
    let not_incr =  [9,8,8,3,3];

    assert!(is_ordered(&incr, Increasing));
    assert!(!is_ordered(&not_decr, Increasing));

    assert!(is_ordered(&decr, Decreasing));
    assert!(!is_ordered(&not_incr, Decreasing));

    assert!(is_ordered(&incr, NotDecreasing));
    assert!(is_ordered(&not_decr, NotDecreasing));
    assert!(!is_ordered(&decr, NotDecreasing));

    assert!(is_ordered(&decr, NotIncreasing));
    assert!(is_ordered(&not_incr, NotIncreasing));
    assert!(!is_ordered(&incr, NotIncreasing));
}
```
*/
pub fn is_ordered<I, T>(iter: I, order: Order) -> bool
where
    I: IntoIterator<Item = T>,
    T: Copy + PartialOrd,
{
   let mut iter = PairIterator::from(iter.into_iter());
    match order{
        Order::Increasing => iter.all(|(a,b)| a<b),
        Order::Decreasing => iter.all(|(a,b)| a>b),
        Order::NotIncreasing => iter.all(|(a,b)| a>=b),
        Order::NotDecreasing => iter.all(|(a,b)| a<=b),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use super::Order::*;

    #[test]
    fn increase_empty() {
        let arr: [i32; 0] = [];
        assert!(is_ordered(&arr, Increasing));
    }

    #[test]
    fn increase_single() {
        let arr = [1];
        assert!(is_ordered(&arr, Increasing));
    }

    #[test]
    fn increase_true() {
        let arr = [1, 4, 6, 10];
        assert!(is_ordered(&arr, Increasing));
    }

    #[test]
    fn increase_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_ordered(&arr, Increasing));
    }

    #[test]
    fn increase_equal() {
        let arr = [1, 4, 6, 6, 10];
        assert!(!is_ordered(&arr, Increasing));
    }

    #[test]
    fn decrease_empty() {
        let arr: [i32; 0] = [];
        assert!(is_ordered(&arr, Decreasing));
    }

    #[test]
    fn decrease_single() {
        let arr = [1];
        assert!(is_ordered(&arr, Decreasing));
    }

    #[test]
    fn decrease_true() {
        let arr = [55, 44, 33, 22, 11];
        assert!(is_ordered(&arr, Decreasing));
    }

    #[test]
    fn decrease_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_ordered(&arr, Decreasing));
    }

    #[test]
    fn decrease_equal() {
        let arr = [19, 9, 6, 6, 1];
        assert!(!is_ordered(&arr, Decreasing));
    }

    #[test]
    fn no_increase_empty() {
        let arr: [i32; 0] = [];
        assert!(is_ordered(&arr, NotIncreasing));
    }

    #[test]
    fn no_increase_single() {
        let arr = [1];
        assert!(is_ordered(&arr, NotIncreasing));
    }

    #[test]
    fn no_increase_true() {
        let arr = [44, 33, 22, 11, 0];
        assert!(is_ordered(&arr, NotIncreasing));
    }

    #[test]
    fn no_increase_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_ordered(&arr, NotIncreasing));
    }

    #[test]
    fn no_increase_equal() {
        let arr = [33, 23, 23, 5, 4, 4, 1];
        assert!(is_ordered(&arr, NotIncreasing));
    }

    #[test]
    fn no_decrease_empty() {
        let arr: [i32; 0] = [];
        assert!(is_ordered(&arr, NotDecreasing));
    }

    #[test]
    fn no_decrease_single() {
        let arr = [1];
        assert!(is_ordered(&arr, NotDecreasing));
    }

    #[test]
    fn no_decrease_true() {
        let arr = [1, 6, 9, 10];
        assert!(is_ordered(&arr, NotDecreasing));
    }

    #[test]
    fn no_decrease_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_ordered(&arr, NotDecreasing));
    }

    #[test]
    fn no_decrease_equal() {
        let arr = [4, 6, 8, 8, 9, 10, 10];
        assert!(is_ordered(&arr, NotDecreasing));
    }

    #[test]
    fn nan() {
        let arr = [1.0, 2.0, ::std::f64::NAN];
        assert!(!is_ordered(&arr, Increasing))
    }

    #[test]
    fn inf_pos() {
        let arr = [1.0, 2.0, ::std::f64::INFINITY];
        assert!(is_ordered(&arr, Increasing))
    }

    #[test]
    fn inf_neg() {
        let arr = [::std::f64::NEG_INFINITY, 1.0, 2.0];
        assert!(is_ordered(&arr, Increasing))
    }

    #[test]
    fn strings() {
        let arr = ["a", "b", "c", "c", "d"];
        assert!(!is_ordered(&arr, Increasing));
        assert!(is_ordered(&arr, NotDecreasing));
    }
}
