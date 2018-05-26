use std::cmp::Ordering;
use utils::{partition, partition_rand};

/**
Finds n-th element in an unsorted slice using a custom comparator.

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
use algorithm::search::quick_select_by;

fn main(){
let mut arr = ['d', 'a', 'b', 'h', 'c', 'f', 'e', 'g'];
//find the fifth element in the array after sorting in the ascending order
let fifth = quick_select_by(&mut arr, 4, |a,b| a<b);
assert_eq!(fifth, 'e');
}
```

*/
pub fn quick_select_by<T, F>(arr: &mut [T], n: usize, is_ordered: F) -> T
where
    T: Clone,
    F: FnMut(&T, &T) -> bool+Copy,
{
    quick_select_impl(arr, n, is_ordered, partition)
}

/**
Finds n-th element in an unsorted slice using a custom comparator.

Normally sorting an array and finding the n-th element in it takes O(n*log(n)). But Quick Select
(using an algorithm similar to Quick Sort) can do it with O(n) complexity.
The algorithm requires a mutable slice and rearranges elements in the slice during execution.

This version chooses a random element for partitioning. This makes quick select run a little slower
but reduces the chance of getting the worst case complexity.

**More:** <https://en.wikipedia.org/wiki/Quickselect>

# Complexity

- Average processing complexity: O(n)
- Worst case processing complexity: O(n²)
- Memory complexity: O(1)

# Example

```
extern crate algorithm;
use algorithm::search::quick_select_rand_by;

fn main(){
let mut arr = ['d', 'a', 'b', 'h', 'c', 'f', 'e', 'g'];
//find the fifth element in the array after sorting in the ascending order
let fifth = quick_select_rand_by(&mut arr, 4 ,|a, b| a<b);
assert_eq!(fifth, 'e');
}
```

*/
pub fn quick_select_rand_by<T, F>(arr: &mut [T], n: usize, is_ordered: F) -> T
    where
        T: Clone,
        F: FnMut(&T, &T) -> bool+Copy,
{
    quick_select_impl(arr, n, is_ordered, partition_rand)
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
use algorithm::search::quick_select;

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
        T: Clone + Ord,
{
    quick_select_impl(arr, n, |a, b| a<b, partition)
}

/**
Finds n-th element in an unsorted slice.

Normally sorting an array and finding the n-th element in it takes O(n*log(n)). But Quick Select
(using an algorithm similar to Quick Sort) can do it with O(n) complexity.
The algorithm requires a mutable slice and rearranges elements in the slice during execution.

This version chooses a random element for partitioning. This makes quick select run a little slower
but reduces the chance of getting the worst case complexity.

**More:** <https://en.wikipedia.org/wiki/Quickselect>

# Complexity

- Average processing complexity: O(n)
- Worst case processing complexity: O(n²)
- Memory complexity: O(1)

# Example

```
extern crate algorithm;
use algorithm::search::quick_select_rand;

fn main(){
let mut arr = ['d', 'a', 'b', 'h', 'c', 'f', 'e', 'g'];
//find the fifth element in the array after sorting
let fifth = quick_select_rand(&mut arr, 4);
assert_eq!(fifth, 'e');
}
```
*/
pub fn quick_select_rand<T>(arr: &mut [T], n: usize) -> T
where
    T: Clone + Ord,
{
    quick_select_impl(arr, n, |a, b| a<b, partition_rand)
}

#[inline(always)]
fn quick_select_impl<T, F>(arr: &mut [T], mut n: usize, is_ordered: F, partition: fn(&mut [T], F) -> usize) -> T
where
    T: Clone,
    F: FnMut(&T, &T) -> bool+Copy,
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
        let pivot_idx = partition(&mut arr[lo..hi], is_ordered);
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
        quick_select(&mut arr, 0);
    }

    #[should_panic]
    #[test]
    fn impl_too_big_n() {
        let mut arr = [3, 6, 5];
        quick_select(&mut arr, 3);
    }

    #[test]
    fn impl_find_first() {
        let mut arr = [3, 6, 5, 0, 8, 9, 2, 1, 4, 7];
        let val = quick_select(&mut arr, 0);
        assert_eq!(val, 0)
    }

    #[test]
    fn impl_find_last() {
        let mut arr = [3, 6, 5, 0, 8, 9, 2, 1, 4, 7];
        let val = quick_select(&mut arr, 9);
        assert_eq!(val, 9)
    }

    #[test]
    fn impl_find_mid() {
        let mut arr = [3, 6, 5, 0, 8, 9, 2, 1, 4, 7];
        let val = quick_select(&mut arr, 5);
        assert_eq!(val, 5)
    }

    #[test]
    fn impl_single() {
        let mut arr = [3];
        let val = quick_select(&mut arr, 0);
        assert_eq!(val, 3)
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
