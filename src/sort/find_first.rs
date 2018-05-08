use std::iter::{Iterator, IntoIterator};

/*
To follow the DRY rule this macro is created.
It works a a template for further implementations.
Different versions differ only in a operator used for comparing values.
*/
macro_rules! find_first {
    ($it: ident, $oper: tt) => { {
        let mut it = $it.into_iter();
        let mut prev = match it.next() {
            Option::None => return None,
            Option::Some(first) => first
        };
        for (i, curr) in it.enumerate() {
            if curr $oper prev {
                return Some(i+1);
            }
            prev = curr;
        }
        None
    }}
}

/**
In a weakly ascending collection it finds index of the first element that is strictly descending.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::first_desc;

fn main() {
    let arr_desc = [3, 5, 7, 7, 7, 4];
    let arr_sorted = [1, 4, 7, 8, 9];
    assert_eq!(first_desc(arr_desc.iter()), Some(5));
    assert_eq!(first_desc(arr_sorted.iter()), None);
}
```
*/
pub fn first_desc<I, T>(iter: I) -> Option<usize> where I: IntoIterator<Item=T>, T:PartialOrd {
    find_first!(iter, <)
}

/**
In a strictly ascending collection it finds index of the first element that is weakly descending.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::first_desc_weak;

fn main() {
    let arr_desc = [3, 5, 7, 7, 7, 4];
    let arr_sorted = [1, 4, 7, 8, 9];
    assert_eq!(first_desc_weak(arr_desc.iter()), Some(3));
    assert_eq!(first_desc_weak(arr_sorted.iter()), None);
}
```
*/
pub fn first_desc_weak<I, T>(iter: I) -> Option<usize> where I: IntoIterator<Item=T>, T:PartialOrd {
    find_first!(iter, <=)
}

/**
In a weakly descending collection it finds index of the first element that is strictly ascending.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::first_asc_weak;

fn main() {
    let arr_asc = [9, 8, 8, 8, 4, 5, 2, 1];
    let arr_sorted = [9, 8, 7, 6, 4, 1];
    assert_eq!(first_asc_weak(arr_asc.iter()), Some(2));
    assert_eq!(first_asc_weak(arr_sorted.iter()), None);
}
```
*/
pub fn first_asc_weak<I, T>(iter: I) -> Option<usize> where I: IntoIterator<Item=T>, T:PartialOrd {
    find_first!(iter, >=)
}

/**
In a strictly descending collection it finds index of the first element that is weakly ascending.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::first_asc;

fn main() {
    let arr_asc = [9, 8, 8, 8, 4, 5, 2, 1];
    let arr_sorted = [9, 8, 7, 7, 4, 1];
    assert_eq!(first_asc(arr_asc.iter()), Some(5));
    assert_eq!(first_asc(arr_sorted.iter()), None);
}
```
*/
pub fn first_asc<I, T>(iter: I) -> Option<usize> where I: IntoIterator<Item=T>, T:PartialOrd {
    find_first!(iter, >)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desc_empty() {
        let arr: [i32; 0] = [];
        assert!(first_desc(arr.iter()).is_none());
    }

    #[test]
    fn desc_single() {
        let arr = [5];
        assert!(first_desc(arr.iter()).is_none());
    }

    #[test]
    fn desc_found() {
        let arr = [5, 7, 8, 10, 4, 11, 12];
        assert_eq!(first_desc(arr.iter()), Some(4));
    }

    #[test]
    fn asc_empty() {
        let arr: [i32; 0] = [];
        assert!(first_asc(arr.iter()).is_none());
    }

    #[test]
    fn asc_single() {
        let arr = [5];
        assert!(first_asc(arr.iter()).is_none());
    }

    #[test]
    fn asc_found() {
        let arr = [44, 33, 22, 88, 11];
        assert_eq!(first_asc(arr.iter()), Some(3));
    }


    #[test]
    fn desc_weak_empty() {
        let arr: [i32; 0] = [];
        assert!(first_desc_weak(arr.iter()).is_none());
    }

    #[test]
    fn desc_weak_single() {
        let arr = [5];
        assert!(first_desc_weak(arr.iter()).is_none());
    }

    #[test]
    fn desc_weak_found() {
        let arr = [5, 7, 8, 10, 10, 11, 12];
        assert_eq!(first_desc_weak(arr.iter()), Some(4));
    }

    #[test]
    fn asc_weak_empty() {
        let arr: [i32; 0] = [];
        assert!(first_asc_weak(arr.iter()).is_none());
    }

    #[test]
    fn asc_weak_single() {
        let arr = [5];
        assert!(first_asc_weak(arr.iter()).is_none());
    }

    #[test]
    fn asc_weak_found() {
        let arr = [44, 33, 22, 22, 11];
        assert_eq!(first_asc_weak(arr.iter()), Some(3));
    }
}