/*!
Quickly finds element in a sorted array.

**More:** <https://en.wikipedia.org/wiki/Binary_search_algorithm>

# Complexity

- Processing omplexity: O(log(n))
- Memory complexity: O(1)
*/

use std::cmp::Ordering;

/**
Performs binary search in a sorted slice using provided comparator.

**More:** <https://en.wikipedia.org/wiki/Binary_search_algorithm>

# Complexity

- Processing omplexity: O(log(n))
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::search::binary_search::binary_search_by;

fn main() {
    let arr = [0, 3, 7, 8, 11, 13, 22];

    assert_eq!(binary_search_by(&arr, |ref val| (&11).cmp(*val)), Some(4));
    assert_eq!(binary_search_by(&arr, |ref val| (*val).cmp(&12)), None);
}
```
*/
pub fn binary_search_by<'a, T, F>(arr: &'a [T], mut cmp: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> Ordering,
{
    let mut lo: isize = 0;
    let mut hi: isize = arr.len() as isize -1;
    while lo <= hi {
        let mid = lo + (hi-lo)/2;
        match cmp(unsafe{arr.get_unchecked(mid as usize)}) {
            Ordering::Less => hi = mid -1,
            Ordering::Greater => lo = mid+1,
            Ordering::Equal => return Option::Some(mid as usize)
        }
    }
    Option::None
}

/**
Performs binary search in a slice sorted in the ascending order.

**More:** <https://en.wikipedia.org/wiki/Binary_search_algorithm>

# Complexity

- Processing omplexity: O(log(n))
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::search::binary_search::binary_search_asc;

fn main() {
    let arr = [0, 3, 7, 8, 11, 13, 22];

    assert_eq!(binary_search_asc(&arr, &11), Some(4));
    assert_eq!(binary_search_asc(&arr, &12), None);
}
```
*/
pub fn binary_search_asc<'a, T>(arr: &'a[T], val: &T) -> Option<usize>
where
    T: Ord,
{
    binary_search_by(arr, |ref t| val.cmp(&t))
}

/**
Performs binary search in a slice sorted in the descending order.

**More:** <https://en.wikipedia.org/wiki/Binary_search_algorithm>

# Complexity

- Processing omplexity: O(log(n))
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::search::binary_search::binary_search_desc;

fn main() {
    let arr = [88, 77, 66, 55, 44, 33, 22, 11];

    assert_eq!(binary_search_desc(&arr, &33), Some(5));
    assert_eq!(binary_search_desc(&arr, &34), None);
}
```
*/
pub fn binary_search_desc<'a, T>(arr: &'a [T], val: &T) -> Option<usize>
where
    T: Ord,
{
    binary_search_by(arr, |ref t| t.cmp(&val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asc_empty() {
        let arr: [i32; 0] = [];
        assert!(binary_search_asc(&arr, &7).is_none());
    }

    #[test]
    fn asc_one() {
        let arr = [5];
        assert!(binary_search_asc(&arr, &7).is_none());
    }

    #[test]
    fn asc_found() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(binary_search_asc(&arr, &12), Some(3));
    }
    #[test]
    fn asc_too_small() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(binary_search_asc(&arr, &3), None);
    }

    #[test]
    fn asc_not_found() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(binary_search_asc(&arr, &13), None);
    }

    #[test]
    fn desc_empty() {
        let arr: [i32; 0] = [];
        assert!(binary_search_desc(&arr, &7).is_none());
    }

    #[test]
    fn desc_one() {
        let arr = [5];
        assert!(binary_search_desc(&arr, &7).is_none());
    }

    #[test]
    fn desc_found() {
        let arr = [44, 33, 22, 9, 7, 4, 2, 1];
        assert_eq!(binary_search_desc(&arr, &2), Some(6));
    }

    #[test]
    fn desc_not_found() {
        let arr = [44, 33, 22, 9, 7, 4, 2, 1];
        assert_eq!(binary_search_asc(&arr, &66), None);
    }
}