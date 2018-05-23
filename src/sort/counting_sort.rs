use super::super::collections::FastCounter;
use std::iter::FromIterator;
use std::hash::Hash;
use std::cmp::Ordering;
use sort::SortingOrder;

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
use algorithm::sort::counting_sort_by;

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
use algorithm::sort::counting_sort;
use algorithm::sort::SortingOrder::*;

fn main() {
    let mut asc = ['a', 'b', 'c', 'a', 'b', 'c', 'c', 'c'];
    counting_sort(&mut asc, Ascending);
    let expected = ['a', 'a', 'b', 'b', 'c', 'c', 'c', 'c'];
    assert_eq!(asc, expected);

    let mut desc = ['c','a', 'b', 'c', 'd', 'a', 'b', 'c', 'c', 'c'];
    counting_sort(&mut desc, Descending);
    let expected = ['d', 'c', 'c', 'c', 'c', 'c', 'b', 'b', 'a', 'a'];
    assert_eq!(desc, expected);
}
```
*/
pub fn counting_sort<T>(arr: &mut [T], order: SortingOrder)
where
    T: Hash + Ord + Eq + Clone,
{
    let counter: FastCounter<T> = FastCounter::from_iter(arr.iter().map(|ref e| e.clone()));
    let mut elems: Vec<(T, usize)> = Vec::from_iter(counter.into_iter());
    match order{
        SortingOrder::Ascending => elems.sort_unstable(),
        SortingOrder::Descending => elems.sort_unstable_by(|ref a, ref b| b.cmp(&a))
    };
    fill_arr(arr, elems);
}



#[cfg(test)]
mod tests {
    use super::*;
    use super::SortingOrder::*;

    #[test]
    fn asc() {
        let mut arr = ['a', 'b', 'c', 'a', 'b', 'c', 'c', 'c'];
        counting_sort(&mut arr, Ascending);
        let expected = ['a', 'a', 'b', 'b', 'c', 'c', 'c', 'c'];
        assert_eq!(arr, expected);
    }

    #[test]
    fn desc() {
        let mut arr = ['c', 'a', 'b', 'c', 'd', 'a', 'b', 'c', 'c', 'c'];
        counting_sort(&mut arr, Descending);
        let expected = ['d', 'c', 'c', 'c', 'c', 'c', 'b', 'b', 'a', 'a'];
        assert_eq!(arr, expected);
    }
}
