use sort::{shuffle, SortingOrder};
use utils::partition;

fn quick_sort_impl<T, F>(arr: &mut [T], cmp: F)
    where
        F: FnMut(&T, &T) -> bool + Copy
{
    if arr.len() >=2 {
        let pivot_idx = partition(arr, cmp);
        quick_sort_impl(&mut arr[..pivot_idx], cmp);
        quick_sort_impl(&mut arr[pivot_idx+1..], cmp);
    }
}

/**
Sorts a slice using a custom comparator.

Although there is a default Rust ```slice.sort_by()``` function for sorting slices,
its implementation is based on merge sort and requires allocation of memory and performing complex
operations. This implementation was chosen because of safety - the default Rust sorting algorithm
has only O(log(n)) worst case complexity. This is much better than the worst case complexity of
quick sort - O(n²). However in the average case quick sort wins with any other sorting algorithm.

The comparator is a function that decides if the given two values are correctly ordered.

**More:** <https://en.wikipedia.org/wiki/Quicksort>

# Complexity

- Average processing complexity: O(n*log(n))
- Worst case complexity: O(n²)
- Memory complexity: O(1)

# Example:
```
extern crate algorithm;
use algorithm::sort::quick_sort_by;
fn main(){
    let mut arr = [5.0,3.0,1.0,4.0,2.0];
    quick_sort_by(&mut arr, |a,b| a<b);
    assert_eq!(arr, [1.0, 2.0, 3.0, 4.0, 5.0]);
}
```
*/
pub fn quick_sort_by<T, F>(arr: &mut [T], cmp: F)
    where
        F: FnMut(&T, &T) -> bool + Copy
{
    quick_sort_impl(arr, cmp)
}

/**
Sorts a slice using a custom comparator.

Although there is a default Rust ```slice.sort_by()``` function for sorting slices,
its implementation is based on merge sort and requires allocation of memory and performing complex
operations. This implementation was chosen because of safety - the default Rust sorting algorithm
has only O(log(n)) worst case complexity. This is much better than the worst case complexity of
quick sort - O(n²). However in the average case quick sort wins with any other sorting algorithm.

The comparator is a function that decides if the given two values are correctly ordered.


This function additionally shuffles input slice before sorting.
Quick sort works poorly with sorted slices having O(n²) processing complexity in this case.
Shuffling makes quick sort work correctly also with sorted slices. Shuffling has only O(n)
complexity, so it does not influence the general quick sort complexity.

**More:** <https://en.wikipedia.org/wiki/Quicksort>

# Complexity

- Average processing complexity: O(n*log(n))
- Worst case complexity: O(n²)
- Memory complexity: O(1)

# Example:
```
extern crate algorithm;
use algorithm::sort::quick_sort_rand_by;
fn main(){
    let mut arr = [5.0,3.0,1.0,4.0,2.0];
    quick_sort_rand_by(&mut arr, |a,b| a<b);
    assert_eq!(arr, [1.0, 2.0, 3.0, 4.0, 5.0]);
}
```
*/
pub fn quick_sort_rand_by<T, F>(arr: &mut [T], cmp: F)
    where
        F: FnMut(&T, &T) -> bool + Copy
{
    shuffle(arr);
    quick_sort_impl(arr, cmp);
}


/**
Sorts a slice.

Although there is a default Rust ```slice.sort_by()``` function for sorting slices,
its implementation is based on merge sort and requires allocation of memory and performing complex
operations. This implementation was chosen because of safety - the default Rust sorting algorithm
has only O(log(n)) worst case complexity. This is much better than the worst case complexity of
quick sort - O(n²). However in the average case quick sort wins with any other sorting algorithm.

**More:** <https://en.wikipedia.org/wiki/Quicksort>

# Complexity

- Average processing complexity: O(n*log(n))
- Worst case complexity: O(n²)
- Memory complexity: O(1)

# Example:
```
extern crate algorithm;
use algorithm::sort::quick_sort;
use algorithm::sort::SortingOrder::*;
fn main(){
    let mut arr = [5,3,1,4,2];
    quick_sort(&mut arr, Ascending);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}
```
*/
pub fn quick_sort<T>(arr: &mut [T], order: SortingOrder)
    where T: Ord
{
    match order{
        SortingOrder::Ascending => quick_sort_impl(arr, |a, b|a<b),
        SortingOrder::Descending => quick_sort_impl(arr, |a, b| a>b)
    }
}

/**
Sorts a slice.

Although there is a default Rust ```slice.sort_by()``` function for sorting slices,
its implementation is based on merge sort and requires allocation of memory and performing complex
operations. This implementation was chosen because of safety - the default Rust sorting algorithm
has only O(log(n)) worst case complexity. This is much better than the worst case complexity of
quick sort - O(n²). However in the average case quick sort wins with any other sorting algorithm.

This function additionally shuffles input slice before sorting.
Quick sort works poorly with sorted slices having O(n²) processing complexity in this case.
Shuffling makes quick sort work correctly also with sorted slices. Shuffling has only O(n)
complexity, so it does not influence the general quick sort complexity.

**More:** <https://en.wikipedia.org/wiki/Quicksort>

# Complexity

- Average processing complexity: O(n*log(n))
- Worst case complexity: O(n²)
- Memory complexity: O(1)

# Example:
```
extern crate algorithm;
use algorithm::sort::quick_sort_rand;
use algorithm::sort::SortingOrder::*;
fn main(){
    let mut arr = [5,3,1,4,2];
    quick_sort_rand(&mut arr, Ascending);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}
```
*/
pub fn quick_sort_rand<T>(arr: &mut [T], order: SortingOrder)
    where T: Ord
{
    shuffle(arr);
    quick_sort(arr, order);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::SortingOrder::*;

    #[test]
    fn simple_by(){
        let mut arr = [5,3,1,2,4];
        quick_sort_by(&mut arr, |a, b| a<b);
        let expected = [1,2,3,4,5];
        assert_eq!(arr, expected);
    }

    #[test]
    fn complex_by(){
        let mut arr = [5,3,1,2,4, 7,7,10,6,8,9];
        quick_sort_by(&mut arr, |a, b| a>b);
        let expected = [10,9,8,7,7,6,5,4,3,2,1];
        assert_eq!(arr, expected);
    }

    #[test]
    fn asc() {
        let mut arr = [100,60,30,10,0, 20, 90, 40, 70, 50, 80];
        let expected = [0,10,20,30,40,50,60,70,80,90,100];
        quick_sort(&mut arr, Ascending);
        assert_eq!(arr, expected);
    }

    #[test]
    fn desc() {
        let mut arr = [100,60,30,10,0, 20, 90, 40, 70, 50, 80];
        let expected = [100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0];
        quick_sort(&mut arr, Descending);
        assert_eq!(arr, expected);
    }

    #[test]
    fn shuffle_sort() {
        let mut arr = [1,2,3,4,5,6,7,8,9,10];
        shuffle(&mut arr);
        quick_sort(&mut arr, Ascending);
        let expected = [1,2,3,4,5,6,7,8,9,10];
        assert_eq!(arr, expected);
    }
}