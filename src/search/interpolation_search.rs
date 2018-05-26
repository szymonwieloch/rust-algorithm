use std::cmp::Ordering;
use sort::SortingOrder;

/**
Quickly finds an element in an ascending slice.

Interpolation search is similar to binary search but for collections with uniform
distribution has a better complexity. If the collection does not have uniform distribution,
interpolation search is slower than binary search and has O(n) complexity in the worst case.

**More:** <https://en.wikipedia.org/wiki/Interpolation_search>

# Complexity

- Average processing complexity (for uniform distribution): O(log(log(n)))
- Worst case processing complexity: O(n)
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::search::interpolation_search;
use algorithm::sort::SortingOrder::*;

fn main(){
    let arr = [5, 7, 8, 12, 22, 33];
    assert_eq!(interpolation_search(&arr, 12, Ascending), Some(3));
    assert_eq!(interpolation_search(&arr, 13, Ascending), None);
}
```
*/
pub fn interpolation_search<'a, T>(arr: &'a [T], val: T, order: SortingOrder) -> Option<usize>
    where T: Ord + Clone,
          f64: From<T>
{
    match order{
        SortingOrder::Ascending => interpolation_search_impl(arr, val, |ref a, ref b| a.cmp(&b)),
        SortingOrder::Descending => interpolation_search_impl(arr, val, |ref a, ref b| b.cmp(&a))
    }
}

/**
Quickly finds an element in a slice using a custom comparator.

 **More:** <https://en.wikipedia.org/wiki/Interpolation_search>

 # Complexity

 - Average processing complexity: O(log(log(n)))
 - Worst case processing complexity: O(n)
 - Memory complexity: O(1)

 # Example

```
extern crate algorithm;
use algorithm::search::interpolation_search_by;
use std::cmp::Ordering;

fn cmp(a: &f64, b: &f64) -> Ordering
{
    a.partial_cmp(b).unwrap()
}

fn main(){
    let mut arr = [55.0, 77.0, 22.0, 33.0, 88.0];
    arr.sort_by(cmp);
     assert_eq!(interpolation_search_by(&arr, 22.0, cmp), Some(0));
    assert_eq!(interpolation_search_by(&arr, 44.0, cmp), None);
}
```
*/
pub fn interpolation_search_by<'a, T, F>(arr: &'a [T], val: T, cmp: F) -> Option<usize>
    where
        T: Clone,
        f64: From<T>,
        F: FnMut(&T, &T) -> Ordering
{
    interpolation_search_impl(arr, val, cmp)
}

#[inline(always)]
fn interpolation_search_impl<'a, T, F>(arr: &'a [T], val: T, mut cmp: F) -> Option<usize>
where
    T: Clone,
     f64: From<T>,
    F: FnMut(&T, &T) -> Ordering

{
    if arr.is_empty(){
        return None;
    }
    let mut lo = 0usize;
    let mut hi = arr.len() -1;
    let val_f64 = f64::from(val.clone());
    while lo <= hi && cmp(&val, &arr[lo]) != Ordering::Less && cmp(&val, &arr[hi]) != Ordering::Greater
    {
        // Probing the position with keeping
        // uniform distribution in mind.
        let y_lo = f64::from(arr[lo].clone());
        let x_diff = (hi-lo) as f64;
        let y_diff = f64::from(arr[hi].clone()) - y_lo;
        let val_diff: f64 = val_f64 - y_lo;

        let mid =  lo + ((x_diff *(val_diff/y_diff)) as usize);

        match cmp(&val, &arr[mid]) {
            Ordering::Less => hi = mid -1,
            Ordering::Greater => lo = mid+1,
            Ordering::Equal => return Option::Some(mid)
        }
    }
    Option::None
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::SortingOrder::*;

    #[test]
    fn asc_empty() {
        let arr: [i32; 0] = [];
        assert!(interpolation_search(&arr, 7, Ascending).is_none());
    }

    #[test]
    fn asc_one() {
        let arr = [5];
        assert!(interpolation_search(&arr, 7, Ascending).is_none());
    }

    #[test]
    fn asc_found() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(interpolation_search(&arr, 12, Ascending), Some(3));
    }
    #[test]
    fn asc_too_small() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(interpolation_search(&arr, 3, Ascending), None);
    }

    #[test]
    fn asc_too_big() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(interpolation_search(&arr, 44, Ascending), None);
    }

    #[test]
    fn asc_not_found() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(interpolation_search(&arr, 13, Ascending), None);
    }
}