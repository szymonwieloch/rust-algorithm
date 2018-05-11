use std::iter::{Iterator, IntoIterator};

/**
Checks if the provided collection is sorted using provided comparator.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

#Example
```
extern crate algorithm;
use algorithm::sort::is_sorted_by;

fn main() {
    let arr_sorted = [3.0,5.0,7.0,9.0];
    let arr_random = [1.0, 7.0, 6.0, 3.0, 0.0];
    assert!(is_sorted_by(&arr_sorted, |&a, &b| b>a));
    assert!(!is_sorted_by(&arr_random, |&a, &b| b>a));
}
```
*/
pub fn is_sorted_by<'a, I, T, F>(iter: I, mut is_ordered:F) -> bool where I: IntoIterator<Item=&'a T>, T:'a , F: FnMut(&T, &T)->bool{
    let mut it = iter.into_iter();
    let mut prev = match it.next() {
        Option::None => return true,
        Option::Some(first) => first
    };
    for curr in it {
        if  ! is_ordered(&prev, &curr){
            return false;
        }
        prev = curr;
    }
    true
}

/**
Checks if the provided collection is sorted in ascending order.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

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
pub fn is_sorted_asc<'a, I, T>(iter: I) -> bool where I: IntoIterator<Item=&'a T>, T:'a+PartialOrd{
    is_sorted_by(iter, |ref a, ref b| a<b)
}

/**
Checks if the provided collection is sorted in descending order.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

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
pub fn is_sorted_desc<'a, I, T>(iter: I) -> bool where I: IntoIterator<Item=&'a T>, T:'a+PartialOrd{
    is_sorted_by(iter, |ref a, ref b| a>b)
}

/**
Checks if the provided collection is sorted in the weakly descending order.

Weakly ordering allows subsequent elements to be equal.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

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
pub fn is_sorted_desc_weak<'a, I, T>(iter: I) -> bool where I: IntoIterator<Item=&'a T>, T:'a+PartialOrd{
    is_sorted_by(iter, |ref a, ref b| a>=b)
}

/**
Checks if the provided collection is sorted in the weakly ascending order.

Weakly ordering allows subsequent elements to be equal.

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

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
pub fn is_sorted_asc_weak<'a, I, T>(iter: I) -> bool where I: IntoIterator<Item=&'a T>, T:'a+PartialOrd{
    is_sorted_by(iter, |ref a, ref b| a<=b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increase_empty() {
        let arr :[i32;0] = [];
        assert!(is_sorted_asc(&arr));
    }

    #[test]
    fn increase_single() {
        let arr = [1];
        assert!(is_sorted_asc(&arr));
    }

    #[test]
    fn increase_true() {
        let arr = [1, 4, 6, 10];
        assert!(is_sorted_asc(&arr));
    }

    #[test]
    fn increase_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_sorted_asc(&arr));
    }

    #[test]
    fn increase_equal() {
        let arr = [1, 4, 6, 6, 10];
        assert!(!is_sorted_asc(&arr));
    }

    #[test]
    fn decrease_empty() {
        let arr :[i32;0] = [];
        assert!(is_sorted_desc(&arr));
    }

    #[test]
    fn decrease_single() {
        let arr = [1];
        assert!(is_sorted_desc(&arr));
    }

    #[test]
    fn decrease_true() {
        let arr = [55, 44, 33, 22, 11];
        assert!(is_sorted_desc(&arr));
    }

    #[test]
    fn decrease_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_sorted_desc(&arr));
    }

    #[test]
    fn decrease_equal() {
        let arr = [19, 9, 6, 6, 1];
        assert!(!is_sorted_desc(&arr));
    }

    #[test]
    fn no_increase_empty() {
        let arr :[i32;0] = [];
        assert!(is_sorted_desc_weak(&arr));
    }

    #[test]
    fn no_increase_single() {
        let arr = [1];
        assert!(is_sorted_desc_weak(&arr));
    }

    #[test]
    fn no_increase_true() {
        let arr = [44, 33, 22, 11, 0];
        assert!(is_sorted_desc_weak(&arr));
    }

    #[test]
    fn no_increase_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_sorted_desc_weak(&arr));
    }

    #[test]
    fn no_increase_equal() {
        let arr = [33, 23, 23, 5, 4, 4, 1];
        assert!(is_sorted_desc_weak(&arr));
    }

    #[test]
    fn no_decrease_empty() {
        let arr :[i32;0] = [];
        assert!(is_sorted_asc_weak(&arr));
    }

    #[test]
    fn no_decrease_single() {
        let arr = [1];
        assert!(is_sorted_asc_weak(&arr));
    }

    #[test]
    fn no_decrease_true() {
        let arr = [1, 6, 9, 10];
        assert!(is_sorted_asc_weak(&arr));
    }

    #[test]
    fn no_decrease_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_sorted_asc_weak(&arr));
    }

    #[test]
    fn no_decrease_equal() {
        let arr = [4, 6, 8, 8, 9, 10, 10];
        assert!(is_sorted_asc_weak(&arr));
    }

    #[test]
    fn nan(){
        let arr = [1.0, 2.0, ::std::f64::NAN];
        assert!(!is_sorted_asc(&arr))
    }

    #[test]
    fn inf_pos() {
        let arr = [1.0, 2.0, ::std::f64::INFINITY];
        assert!(is_sorted_asc(&arr))
    }

    #[test]
    fn inf_neg() {
        let arr = [ ::std::f64::NEG_INFINITY, 1.0, 2.0];
        assert!(is_sorted_asc(&arr))
    }
}