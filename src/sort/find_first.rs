use std::iter::{Iterator, IntoIterator};


/**
Finds index of the first element that is not sorted using a provided comparator.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::find_first_by;

fn main() {
    let arr_desc = [3, 5, 7, 7, 7, 4];
    let arr_sorted = [1, 4, 7, 8, 9];
    assert_eq!(find_first_by(&arr_desc, |&a, &b| a<=b), Some(5));
    assert_eq!(find_first_by(&arr_sorted, |&a, &b| a<=b), None);
}
```
*/
pub fn find_first_by<'a, I, T, F>(iter: I, mut is_ordered: F) -> Option<usize> where I: IntoIterator<Item=&'a T>, F: FnMut(&T, &T)->bool, T:'a{
    let mut it = iter.into_iter();
    let mut prev = match it.next() {
        Option::None => return None,
        Option::Some(first) => first
    };
    for (i, curr) in it.enumerate() {
        if ! is_ordered(prev, curr) {
            return Some(i+1);
        }
        prev = curr;
    }
    None
}

/**
In a weakly ascending collection it finds index of the first element that is strictly descending.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::find_first_desc;

fn main() {
    let arr_desc = [3, 5, 7, 7, 7, 4];
    let arr_sorted = [1, 4, 7, 8, 9];
    assert_eq!(find_first_desc(arr_desc.iter()), Some(5));
    assert_eq!(find_first_desc(arr_sorted.iter()), None);
}
```
*/
pub fn find_first_desc<'a, I, T>(iter: I) -> Option<usize> where I: IntoIterator<Item=&'a T>, T:PartialOrd, T:'a {
    find_first_by(iter, |ref a, ref b| a<=b)
}

/**
In a strictly ascending collection it finds index of the first element that is weakly descending.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::find_first_wdesc;

fn main() {
    let arr_desc = [3, 5, 7, 7, 7, 4];
    let arr_sorted = [1, 4, 7, 8, 9];
    assert_eq!(find_first_wdesc(arr_desc.iter()), Some(3));
    assert_eq!(find_first_wdesc(arr_sorted.iter()), None);
}
```
*/
pub fn find_first_wdesc<'a, I, T>(iter: I) -> Option<usize> where I: IntoIterator<Item=&'a T>, T:PartialOrd, T:'a {
    find_first_by(iter, |ref a, ref b| a<b)
}

/**
In a weakly descending collection it finds index of the first element that is strictly ascending.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::find_first_wasc;

fn main() {
    let arr_asc = [9, 8, 8, 8, 4, 5, 2, 1];
    let arr_sorted = [9, 8, 7, 6, 4, 1];
    assert_eq!(find_first_wasc(arr_asc.iter()), Some(2));
    assert_eq!(find_first_wasc(arr_sorted.iter()), None);
}
```
*/
pub fn find_first_wasc<'a, I, T>(iter: I) -> Option<usize> where I: IntoIterator<Item=&'a T>, T:PartialOrd, T:'a {
    find_first_by(iter, |ref a, ref b| a>b)
}

/**
In a strictly descending collection it finds index of the first element that is weakly ascending.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::find_first_asc;

fn main() {
    let arr_asc = [9, 8, 8, 8, 4, 5, 2, 1];
    let arr_sorted = [9, 8, 7, 7, 4, 1];
    assert_eq!(find_first_asc(arr_asc.iter()), Some(5));
    assert_eq!(find_first_asc(arr_sorted.iter()), None);
}
```
*/
pub fn find_first_asc<'a, I, T>(iter: I) -> Option<usize> where I: IntoIterator<Item=&'a T>, T:PartialOrd, T:'a {
    find_first_by(iter, |ref a, ref b| a>=b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desc_empty() {
        let arr: [i32; 0] = [];
        assert!(find_first_desc(arr.iter()).is_none());
    }

    #[test]
    fn desc_single() {
        let arr = [5];
        assert!(find_first_desc(arr.iter()).is_none());
    }

    #[test]
    fn desc_found() {
        let arr = [5, 7, 8, 10, 4, 11, 12];
        assert_eq!(find_first_desc(arr.iter()), Some(4));
    }

    #[test]
    fn asc_empty() {
        let arr: [i32; 0] = [];
        assert!(find_first_asc(arr.iter()).is_none());
    }

    #[test]
    fn asc_single() {
        let arr = [5];
        assert!(find_first_asc(arr.iter()).is_none());
    }

    #[test]
    fn asc_found() {
        let arr = [44, 33, 22, 88, 11];
        assert_eq!(find_first_asc(arr.iter()), Some(3));
    }


    #[test]
    fn wdesc_empty() {
        let arr: [i32; 0] = [];
        assert!(find_first_wdesc(arr.iter()).is_none());
    }

    #[test]
    fn wdesc_single() {
        let arr = [5];
        assert!(find_first_wdesc(arr.iter()).is_none());
    }

    #[test]
    fn wdesc_found() {
        let arr = [5, 7, 8, 10, 10, 11, 12];
        assert_eq!(find_first_wdesc(arr.iter()), Some(4));
    }

    #[test]
    fn wasc_empty() {
        let arr: [i32; 0] = [];
        assert!(find_first_wasc(arr.iter()).is_none());
    }

    #[test]
    fn wasc_single() {
        let arr = [5];
        assert!(find_first_wasc(arr.iter()).is_none());
    }

    #[test]
    fn wasc_found() {
        let arr = [44, 33, 22, 22, 11];
        assert_eq!(find_first_wasc(arr.iter()), Some(3));
    }
}