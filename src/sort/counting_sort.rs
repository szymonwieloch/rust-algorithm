use super::super::collections::FastCounter;
use std::iter::FromIterator;
use std::hash::Hash;

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
Sorts elements using counting sort in the ascending order.

# Complexity

- Processing complexity: O(n + k*log(k))
- Memory complexity: O(k)

where k - number of unique elements in the provided slice.

# Example

```
extern crate algorithm;
use algorithm::sort::counting_sort_asc;

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

# Complexity

- Processing complexity: O(n + k*log(k))
- Memory complexity: O(k)

where k - number of unique elements in the provided slice.

# Example

```
extern crate algorithm;
use algorithm::sort::counting_sort_desc;

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
