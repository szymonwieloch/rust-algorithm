/*!
Effectively sorts an array that contains a small number of unique elements.

**More:** <https://en.wikipedia.org/wiki/Counting_sort>

# Complexity

- Processing complexity: O(n + k*log(k))
- Memory complexity: O(k)

where k - number of unique elements in the provided slice.
*/

use super::super::collections::FastCounter;
use std::iter::FromIterator;
use std::hash::Hash;
use std::cmp::Ordering;

fn fill_arr<T>(arr: &mut [T], elems: Vec<(T, usize)>)
where
    T: Clone,
{
    let mut curr = arr.as_mut_ptr();
    for (val, count) in elems.into_iter() {
        for _ in 0..count {
            unsafe {
                *curr = val.clone();
                curr = curr.offset(1);
            }
        }
    }
}


/**
Sorts elements using counting sort with the provided comparator.

**More:** <https://en.wikipedia.org/wiki/Counting_sort>

# Complexity

- Processing complexity: O(n + k*log(k))
- Memory complexity: O(k)

where k - number of unique elements in the provided slice.

# Example

```
extern crate algorithm;
use algorithm::sort::counting_sort::counting_sort_by;

fn main() {
    let mut arr = ['a', 'b', 'c', 'a', 'b', 'c', 'c', 'c'];
    counting_sort_by(&mut arr, |&a, &b| b.cmp(&a));
    let expected = ['c', 'c', 'c', 'c', 'b', 'b', 'a', 'a'];
    assert_eq!(arr, expected);
}
```
*/
pub fn counting_sort_by<'a, T, F>(arr: & mut [T], mut cmp: F) where T: 'a+Hash + Eq + Clone, F: FnMut(& T, & T)-> Ordering{
    let counter: FastCounter<T> = FastCounter::from_iter(arr.iter().map(|ref e| e.clone()));
    let mut elems: Vec<(T, usize)> = Vec::from_iter(counter.into_iter());
    elems.sort_unstable_by(|ref a, ref b| cmp(&a.0, &b.0));
    fill_arr(arr, elems);
}

/**
Sorts elements using counting sort in the ascending order.

**More:** <https://en.wikipedia.org/wiki/Counting_sort>

# Complexity

- Processing complexity: O(n + k*log(k))
- Memory complexity: O(k)

where k - number of unique elements in the provided slice.

# Example

```
extern crate algorithm;
use algorithm::sort::counting_sort::counting_sort_asc;

fn main() {
    let mut arr = ['a', 'b', 'c', 'a', 'b', 'c', 'c', 'c'];
    counting_sort_asc(&mut arr);
    let expected = ['a', 'a', 'b', 'b', 'c', 'c', 'c', 'c'];
    assert_eq!(arr, expected);
}
```
*/
pub fn counting_sort_asc<T>(arr: &mut [T])
where
    T: Hash + Ord + Eq + Clone,
{
    let counter: FastCounter<T> = FastCounter::from_iter(arr.iter().map(|ref e| e.clone()));
    let mut elems: Vec<(T, usize)> = Vec::from_iter(counter.into_iter());
    elems.sort_unstable();
    fill_arr(arr, elems);
}

/**
Sorts elements using counting sort in the descending order.

**More:** <https://en.wikipedia.org/wiki/Counting_sort>

# Complexity

- Processing complexity: O(n + k*log(k))
- Memory complexity: O(k)

where k - number of unique elements in the provided slice.

# Example

```
extern crate algorithm;
use algorithm::sort::counting_sort::counting_sort_desc;

fn main() {
    let mut arr = ['c','a', 'b', 'c', 'd', 'a', 'b', 'c', 'c', 'c'];
    counting_sort_desc(&mut arr);
    let expected = ['d', 'c', 'c', 'c', 'c', 'c', 'b', 'b', 'a', 'a'];
    assert_eq!(arr, expected);
}
```
*/
pub fn counting_sort_desc<T>(arr: &mut [T])
where
    T: Hash + Ord + Eq + Clone,
{
    let counter: FastCounter<T> = FastCounter::from_iter(arr.iter().map(|ref e| e.clone()));
    let mut elems: Vec<(T, usize)> = Vec::from_iter(counter.into_iter());
    elems.sort_unstable_by(|a, b| b.cmp(a));
    fill_arr(arr, elems);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asc() {
        let mut arr = ['a', 'b', 'c', 'a', 'b', 'c', 'c', 'c'];
        counting_sort_asc(&mut arr);
        let expected = ['a', 'a', 'b', 'b', 'c', 'c', 'c', 'c'];
        assert_eq!(arr, expected);
    }

    #[test]
    fn desc() {
        let mut arr = ['c', 'a', 'b', 'c', 'd', 'a', 'b', 'c', 'c', 'c'];
        counting_sort_desc(&mut arr);
        let expected = ['d', 'c', 'c', 'c', 'c', 'c', 'b', 'b', 'a', 'a'];
        assert_eq!(arr, expected);
    }
}
