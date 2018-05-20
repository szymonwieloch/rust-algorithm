/*!
Calculates median of the slice.

**More:** <https://en.wikipedia.org/wiki/Median>

# Complexity

- Average processing complexity: O(n)
- Worst processing complexity: O(n**2)
- Memory complexity: 0(1)
*/

use super::super::search::quick_select::{quick_select, quick_select_rand};
use std::ops::{Add, Div};

#[inline(always)]
fn check_len(len: usize) {
    if len == 0 {
        panic!("Cannot calculate median of empty collection");
    }
}

/**
Calculates median using the quick select algorithm.

If the array has an odd number of elements, the middle element is returned.
If the array has an even number of elements, the greater one of the middle two is returned.

The provided mutable slice is used for calculations and its elements are rearranged
during processing.

**More:** <https://en.wikipedia.org/wiki/Median>

# Complexity

- Average processing complexity: O(n)
- Worst processing complexity: O(n**2)
- Memory complexity: 0(1)

# Example

```
extern crate algorithm;
use algorithm::math::median::median;

fn main(){
    let mut odd = [9, 2, 7, 3, 5, 4, 1, 6, 8 ];
    assert_eq!(median(&mut odd), 5);
    let mut even = [9, 2, 7, 3, 5, 4, 1, 6, 8, 10 ];
    assert_eq!(median(&mut even), 6);
}
```
*/
pub fn median<T>(arr: &mut [T]) -> T
where
    T: Clone + PartialOrd,
{
    let len = arr.len();
    check_len(len);
    let mid = len / 2;
    println!("mid {}", mid);
    quick_select(arr, mid)
}

/**
    Calculates median using the randomized quick select algorithm.

    This is a version of ```median()``` function that uses randomized version of quick select.
    It is safer but slower.
*/
pub fn median_rand<T>(arr: &mut [T]) -> T
where
    T: Clone + PartialOrd,
{
    let len = arr.len();
    check_len(len);
    quick_select_rand(arr, len / 2)
}

/**
Calculates median using the quick select algorithm.

If the array has an odd number of elements, the middle element is returned.
If the array has an even number of elements, average of the middle two elements is returned.

The provided mutable slice is used for calculations and its elements are rearranged
during processing.

**More:** <https://en.wikipedia.org/wiki/Median>

# Complexity

- Average processing complexity: O(n)
- Worst processing complexity: O(n**2)
- Memory complexity: 0(1)

# Example

```
extern crate algorithm;
use algorithm::math::median::median_avg;

fn main(){
    let mut odd = [9, 2, 7, 3, 5, 4, 1, 6, 8];
    assert_eq!(median_avg(&mut odd), 5);
    let mut even = [9, 2, 7, 3, 4, 1, 6, 8, 10, 0];
    assert_eq!(median_avg(&mut even), 5);
}
```
*/
pub fn median_avg<T>(arr: &mut [T]) -> T
where
    T: Clone + Ord + Add<Output = T> + Div<Output = T> + From<i32>,
{
    median_avg_impl(arr, quick_select)
}


/**
    Calculates median using the randomized quick select algorithm.

    This is a version of ```median_avg()``` function that uses randomized version of quick select.
    It is safer but slower.
*/
pub fn median_avg_rand<T>(arr: &mut [T]) -> T
where
    T: Clone + Ord + Add<Output = T> + Div<Output = T> + From<i32>,
{
    median_avg_impl(arr, quick_select_rand)
}

fn median_avg_impl<T>(arr: &mut [T], qs: fn(&mut [T], usize) -> T) -> T
where
    T: Clone + Ord + Add<Output = T> + Div<Output = T> + From<i32>,
{
    let len = arr.len();
    check_len(len);
    let mid = qs(arr, len / 2);
    if len % 2 == 1 {
        mid
    } else {
        //after quick_select the array is partially sorted
        //all smaller elements are on the left, all bigger on the right
        let prev = arr[..len / 2].iter().max().unwrap();
        (mid + prev.clone()) / T::from(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        median(&mut arr);
    }

    #[should_panic]
    #[test]
    fn empty_avg() {
        let mut arr: [i32; 0] = [];
        median_avg(&mut arr);
    }

    #[test]
    fn single() {
        let mut arr = [3];
        assert_eq!(median(&mut arr), 3);
    }

    #[test]
    fn single_avg() {
        let mut arr = [3];
        assert_eq!(median_avg(&mut arr), 3);
    }

    #[test]
    fn two() {
        let mut arr = [3, 5];
        assert_eq!(median(&mut arr), 5);
    }

    #[test]
    fn two_avg() {
        let mut arr = [3, 5];
        assert_eq!(median_avg(&mut arr), 4);
    }

    #[test]
    #[ignore]
    fn multiple_odd() {
        let mut arr = [9, 2, 7, 3, 5, 4, 1, 6, 8];
        assert_eq!(median(&mut arr), 5);
    }

    #[test]
    #[ignore]
    fn multiple_even() {
        let mut arr = [9, 2, 7, 3, 5, 4, 1, 6, 8, 0];
        assert_eq!(median(&mut arr), 5);
    }

    #[test]
    #[ignore]
    fn multiple_odd_avg() {
        let mut arr = [9, 2, 7, 3, 5, 4, 1, 6, 8];
        assert_eq!(median_avg(&mut arr), 5);
    }

    #[test]
    #[ignore]
    fn multiple_even_avg() {
        let mut arr = [9, 2, 7, 3, 5, 4, 1, 6, 8, 0];
        assert_eq!(median_avg(&mut arr), 4);
    }
}
