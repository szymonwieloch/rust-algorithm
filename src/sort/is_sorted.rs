use std::iter::{Iterator, IntoIterator};

/*
To follow the DRY rule a macro is created here and then used in 4 versions of functions.
It is actually a template for all functions only with a change in the comparision operator.
This is the best performing solution.
*/
macro_rules! check_sorted {
    ($it:ident, $cond:tt) => {
        {
            let mut it = $it.into_iter();
            let mut prev = match it.next() {
                Option::None => return true,
                Option::Some(first) => first
            };
            for curr in it {
                if  curr $cond prev{
                    return false;
                }
                prev = curr;
            }
            return true;
        }
    };
}

/**
Checks if the provided collection is sorted in ascending order.

Processing complexity: O(n)
Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::is_sorted_asc;

fn main() {
    let arr_sorted = [3,5,7,9];
    let arr_random = [1, 7, 6, 3, 0];
    assert!(is_sorted_asc(arr_sorted.iter()));
    assert!(!is_sorted_asc(arr_random.iter()));
}
```
*/
pub fn is_sorted_asc<I, T>(iter: I) -> bool where I: IntoIterator<Item=T>, T:PartialOrd{
    check_sorted!(iter, <=)
}

/**
Checks if the provided collection is sorted in descending order.

Processing complexity: O(n)
Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::is_sorted_desc;

fn main() {
    let arr_sorted = [9, 8, 6, 5, 3, 0];
    let arr_random = [1, 7, 6, 3, 0];
    assert!(is_sorted_desc(arr_sorted.iter()));
    assert!(!is_sorted_desc(arr_random.iter()));
}
```
*/
pub fn is_sorted_desc<I, T>(iter: I) -> bool where I: IntoIterator<Item=T>, T:PartialOrd{
    check_sorted!(iter, >=)
}

/**
Checks if the provided collection is sorted in the weakly descending order.

Weakly ordering allows subsequent elements to be equal.

Processing complexity: O(n)
Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::is_sorted_desc_weak;

fn main() {
    let arr_sorted = [9, 8, 8, 8, 6, 5, 3, 0];
    let arr_random = [1, 7, 6, 3, 0];
    assert!(is_sorted_desc_weak(arr_sorted.iter()));
    assert!(!is_sorted_desc_weak(arr_random.iter()));
}
```
*/
pub fn is_sorted_desc_weak<I, T>(iter: I) -> bool where I: IntoIterator<Item=T>, T:PartialOrd{
    check_sorted!(iter, >)
}

/**
Checks if the provided collection is sorted in the weakly ascending order.

Weakly ordering allows subsequent elements to be equal.

Processing complexity: O(n)
Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::is_sorted_asc_weak;

fn main() {
    let arr_sorted = [3, 5, 7, 7, 7, 9];
    let arr_random = [1, 7, 6, 3, 0];
    assert!(is_sorted_asc_weak(arr_sorted.iter()));
    assert!(!is_sorted_asc_weak(arr_random.iter()));
}
```
*/
pub fn is_sorted_asc_weak<I, T>(iter: I) -> bool where I: IntoIterator<Item=T>, T:PartialOrd{
    check_sorted!(iter, <)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increase_empty() {
        let arr :[i32;0] = [];
        assert!(is_sorted_asc(arr.iter()));
    }

    #[test]
    fn increase_single() {
        let arr = [1];
        assert!(is_sorted_asc(arr.iter()));
    }

    #[test]
    fn increase_true() {
        let arr = [1, 4, 6, 10];
        assert!(is_sorted_asc(arr.iter()));
    }

    #[test]
    fn increase_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_sorted_asc(arr.iter()));
    }

    #[test]
    fn increase_equal() {
        let arr = [1, 4, 6, 6, 10];
        assert!(!is_sorted_asc(arr.iter()));
    }

    #[test]
    fn decrease_empty() {
        let arr :[i32;0] = [];
        assert!(is_sorted_desc(arr.iter()));
    }

    #[test]
    fn decrease_single() {
        let arr = [1];
        assert!(is_sorted_desc(arr.iter()));
    }

    #[test]
    fn decrease_true() {
        let arr = [55, 44, 33, 22, 11];
        assert!(is_sorted_desc(arr.iter()));
    }

    #[test]
    fn decrease_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_sorted_desc(arr.iter()));
    }

    #[test]
    fn decrease_equal() {
        let arr = [19, 9, 6, 6, 1];
        assert!(!is_sorted_desc(arr.iter()));
    }

    #[test]
    fn no_increase_empty() {
        let arr :[i32;0] = [];
        assert!(is_sorted_desc_weak(arr.iter()));
    }

    #[test]
    fn no_increase_single() {
        let arr = [1];
        assert!(is_sorted_desc_weak(arr.iter()));
    }

    #[test]
    fn no_increase_true() {
        let arr = [44, 33, 22, 11, 0];
        assert!(is_sorted_desc_weak(arr.iter()));
    }

    #[test]
    fn no_increase_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_sorted_desc_weak(arr.iter()));
    }

    #[test]
    fn no_increase_equal() {
        let arr = [33, 23, 23, 5, 4, 4, 1];
        assert!(is_sorted_desc_weak(arr.iter()));
    }

    #[test]
    fn no_decrease_empty() {
        let arr :[i32;0] = [];
        assert!(is_sorted_asc_weak(arr.iter()));
    }

    #[test]
    fn no_decrease_single() {
        let arr = [1];
        assert!(is_sorted_asc_weak(arr.iter()));
    }

    #[test]
    fn no_decrease_true() {
        let arr = [1, 6, 9, 10];
        assert!(is_sorted_asc_weak(arr.iter()));
    }

    #[test]
    fn no_decrease_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_sorted_asc_weak(arr.iter()));
    }

    #[test]
    fn no_decrease_equal() {
        let arr = [4, 6, 8, 8, 9, 10, 10];
        assert!(is_sorted_asc_weak(arr.iter()));
    }
}