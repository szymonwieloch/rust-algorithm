/*!
Finds n-th element in an unsorted slice.

Normally sorting an array and finding the n-th element in it takes O(n*log(n)). But Quick Select
(using an algorithm similar to Quick Sort) can do it with O(n) complexity.
The algorithm requires a mutable slice and rearranges elements in the slice during execution.

**More:** <https://en.wikipedia.org/wiki/Quickselect>

# Complexity

- Average processing complexity: O(n)
- Worst case processing complexity: O(n²)
- Memory complexity: O(1)
*/

use std::cmp::Ordering;
use rand::distributions::Range;
use rand::distributions::IndependentSample;

fn partition<T>(arr: &mut [T], pivot_idx: usize) -> usize
where
    T: Clone + PartialOrd,
{
    let last_idx = arr.len() - 1;
    let pivot_val = unsafe { arr.get_unchecked(pivot_idx) }.clone();
    arr.swap(pivot_idx, last_idx);
    let mut store_idx = 0;
    for i in 0..last_idx {
        if unsafe { arr.get_unchecked(i) } < &pivot_val {
            arr.swap(store_idx, i);
            store_idx += 1;
        }
    }
    arr.swap(store_idx, last_idx);
    store_idx
}

/**
Finds n-th element in an unsorted slice.

Normally sorting an array and finding the n-th element in it takes O(n*log(n)). But Quick Select
(using an algorithm similar to Quick Sort) can do it with O(n) complexity.
The algorithm requires a mutable slice and rearranges elements in the slice during execution.

**More:** <https://en.wikipedia.org/wiki/Quickselect>

# Complexity

- Average processing complexity: O(n)
- Worst case processing complexity: O(n²)
- Memory complexity: O(1)

# Example

```
extern crate algorithm;
use algorithm::search::quick_select::quick_select;

fn main(){
let mut arr = ['d', 'a', 'b', 'h', 'c', 'f', 'e', 'g'];
//find the fifth element in the array after sorting
let fifth = quick_select(&mut arr, 4);
assert_eq!(fifth, 'e');
}
```

*/
pub fn quick_select<T>(arr: &mut [T], n: usize) -> T
where
    T: Clone + PartialOrd,
{
    quick_select_impl(arr, n, |_len| 0)
}

/**
A safer but slower version of quick_select().

This version uses random generator to find the middle element in the array.
This makes it more resistant against hacking attempts but you a perfomance penalty for using
a random generator.
*/
pub fn quick_select_rand<T>(arr: &mut [T], n: usize) -> T
where
    T: Clone + PartialOrd,
{
    let mut rng = ::rand::thread_rng();
    quick_select_impl(arr, n, |len| {
        let between = Range::new(0, len);
        between.ind_sample(&mut rng)
    })
}

fn quick_select_impl<T, F>(arr: &mut [T], mut n: usize, mut choose_pivot: F) -> T
where
    T: Clone + PartialOrd,
    F: FnMut(usize) -> usize,
{
    if n >= arr.len() {
        panic!("n is outside of the array.");
    }

    let mut lo: usize = 0;
    let mut hi: usize = arr.len();
    loop {
        if hi == lo + 1 {
            return unsafe { arr.get_unchecked(lo) }.clone();
        }
        let pivot_idx = partition(&mut arr[lo..hi], choose_pivot(hi - lo));
        match n.cmp(&pivot_idx) {
            Ordering::Greater => {
                lo += pivot_idx + 1;
                n -= pivot_idx + 1;
            }
            Ordering::Less => {
                hi = lo + pivot_idx;
            }
            Ordering::Equal => return unsafe { arr.get_unchecked(lo + n) }.clone(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn impl_empty_arr() {
        let mut arr: [i32; 0] = [];
        quick_select_impl(&mut arr, 0, |_| 0);
    }

    #[should_panic]
    #[test]
    fn impl_too_big_n() {
        let mut arr = [3, 6, 5];
        quick_select_impl(&mut arr, 3, |_| 0);
    }

    #[test]
    fn impl_find_first() {
        let mut arr = [3, 6, 5, 0, 8, 9, 2, 1, 4, 7];
        let val = quick_select_impl(&mut arr, 0, |_| 0);
        assert_eq!(val, 0)
    }

    #[test]
    fn impl_find_last() {
        let mut arr = [3, 6, 5, 0, 8, 9, 2, 1, 4, 7];
        let val = quick_select_impl(&mut arr, 9, |_| 0);
        assert_eq!(val, 9)
    }

    #[test]
    fn impl_find_mid() {
        let mut arr = [3, 6, 5, 0, 8, 9, 2, 1, 4, 7];
        let val = quick_select_impl(&mut arr, 5, |_| 0);
        assert_eq!(val, 5)
    }

    #[test]
    fn impl_single() {
        let mut arr = [3];
        let val = quick_select_impl(&mut arr, 0, |_| 0);
        assert_eq!(val, 3)
    }

    #[test]
    fn partition_simple() {
        let mut arr = [7, 4, 8, 3, 9];
        let idx = partition(&mut arr, 0);
        assert_eq!(arr, [4, 3, 7, 9, 8]);
        assert_eq!(idx, 2)
    }

    #[test]
    fn partition_complex() {
        let mut arr = [5, 4, 1, 2, 8, 9, 4, 6, 6, 8];
        let idx = partition(&mut arr, 0);
        assert_eq!(arr, [4, 1, 2, 4, 5, 9, 8, 6, 6, 8]);
        assert_eq!(idx, 4)
    }

    #[test]
    fn partition_first() {
        let mut arr = [1, 5, 8, 6];
        let idx = partition(&mut arr, 0);
        assert_eq!(arr, [1, 5, 8, 6]);
        assert_eq!(idx, 0)
    }

    #[test]
    fn partition_last() {
        let mut arr = [9, 4, 8, 3, 3];
        let idx = partition(&mut arr, 0);
        assert_eq!(arr, [3, 4, 8, 3, 9]);
        assert_eq!(idx, 4)
    }

    #[test]
    fn qs_letters() {
        let mut arr = ['d', 'a', 'b', 'h', 'c', 'f', 'e', 'g'];
        let fifth = quick_select(&mut arr, 4);
        arr.sort();
        assert_eq!(fifth, 'e');
    }

    #[test]
    fn qs_found_by_partition() {
        let mut arr = [9, 2, 7, 3, 5, 4, 1, 6, 8];
        assert_eq!(quick_select(&mut arr, 4), 5);
    }

}
