use std::cmp::Ordering;
use super::super::sort::SortingOrder;

//this is the actual implementation of binary search
//it is inlined so that the comparator gets inlined too
#[inline(always)]
fn binary_search_impl<'a, T, F>(arr: &'a [T], mut cmp: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> Ordering,
{
    //This check is needed because otherwise hi can be lower than 0
    if arr.is_empty(){
        return None;
    }
    //highest and lowest indexes of the subslice being checked
    let mut lo: usize = 0;
    let mut hi: usize = arr.len() -1;
    while lo <= hi {
        //it is better to use (hi-lo)/2 because writing (hi+2)/2 can lead to a overflow
        let mid = lo + (hi-lo)/2;
        //this i safe, mid is always in the range
        match cmp(unsafe{arr.get_unchecked(mid)}) {
            Ordering::Less => {
                //prevent underflow
                if mid == 0{
                    return None;
                }
                hi = mid -1;
            },
            Ordering::Greater => lo = mid+1,
            Ordering::Equal => return Option::Some(mid)
        }
    }
    Option::None
}



/**
Performs binary search in a sorted slice using provided comparator.

Binary search is a popular way of speeading up search for a given value in a collection.
While normally comparing value after value has O(n) complexity, binary search achieves the same
result in just O(log(n)). It requires previously sorted slice. If the slice is unsorted,
```binary_search()``` returns invalid information.

**More:** <https://en.wikipedia.org/wiki/Binary_search_algorithm>

# Complexity

- Processing omplexity: O(log(n))
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::search::binary_search_by;
use algorithm::sort::SortingOrder::*;

fn main() {
    let arr = [0, 3, 7, 8, 11, 13, 22];

    assert_eq!(binary_search_by(&arr, |ref val| (&11).cmp(*val)), Some(4));
    assert_eq!(binary_search_by(&arr, |ref val| (&12).cmp(*val)), None);
}
```
*/
pub fn binary_search_by<'a, T, F>(arr: &'a [T], cmp: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> Ordering,
{
    binary_search_impl(arr, cmp)
}

/**
Performs binary search in a sorted slice.

Binary search is a popular way of speeading up search for a given value in a collection.
While normally comparing value after value has O(n) complexity, binary search achieves the same
result in just O(log(n)). It requires previously sorted slice. If the slice is unsorted,
```binary_search()``` returns invalid information.

**More:** <https://en.wikipedia.org/wiki/Binary_search_algorithm>

# Complexity

- Processing omplexity: O(log(n))
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::search::binary_search;
use algorithm::sort::SortingOrder::*;

fn main() {
    let asc = [0, 3, 7, 8, 11, 13, 22];
    assert_eq!(binary_search(&asc, &11, Ascending), Some(4));
    assert_eq!(binary_search(&asc, &12, Ascending), None);

    let desc = [100, 88, 74, 53, 24, 12, 1];
    assert_eq!(binary_search(&desc, &24, Descending), Some(4));
    assert_eq!(binary_search(&desc, &23, Descending), None);
}
```
*/
pub fn binary_search<'a, T>(arr: &'a[T], val: &T, order: SortingOrder) -> Option<usize>
where
    T: Ord,
{
    match order{
        SortingOrder::Ascending => binary_search_impl(arr, |ref t| val.cmp(&t)),
        SortingOrder::Descending => binary_search_impl(arr, |ref t| t.cmp(&val)),
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    use super::SortingOrder::*;

    #[test]
    fn asc_empty() {
        let arr: [i32; 0] = [];
        assert!(binary_search(&arr, &7, Ascending).is_none());
    }

    #[test]
    fn asc_one() {
        let arr = [5];
        assert!(binary_search(&arr, &7, Ascending).is_none());
    }

    #[test]
    fn asc_found() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(binary_search(&arr, &12, Ascending), Some(3));
    }
    #[test]
    fn asc_too_small() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(binary_search(&arr, &3, Ascending), None);
    }

    #[test]
    fn asc_not_found() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(binary_search(&arr, &13, Ascending), None);
    }

    #[test]
    fn desc_empty() {
        let arr: [i32; 0] = [];
        assert!(binary_search(&arr, &7, Descending).is_none());
    }

    #[test]
    fn desc_one() {
        let arr = [5];
        assert!(binary_search(&arr, &7, Descending).is_none());
    }

    #[test]
    fn desc_found() {
        let arr = [44, 33, 22, 9, 7, 4, 2, 1];
        assert_eq!(binary_search(&arr, &2, Descending), Some(6));
    }

    #[test]
    fn desc_not_found() {
        let arr = [44, 33, 22, 9, 7, 4, 2, 1];
        assert_eq!(binary_search(&arr, &66, Descending), None);
    }
    /*
    #[test]
    fn first_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search_first(&arr, |&e| true), None);
    }

    #[test]
    fn first_single_found_greater() {
        let arr = [4];
        assert_eq!(binary_search_first(&arr, |&e| e >=4), Some(0));
    }

    #[test]
    fn first_single_found_less() {
        let arr = [4];
        assert_eq!(binary_search_first(&arr, |&e| e <=4), Some(0));
    }

    #[test]
    fn first_single_too_small() {
        let arr = [4];
        assert_eq!(binary_search_first(&arr, |&e| e >4), None);
    }

    #[test]
    fn first_single_too_big() {
        let arr = [4];
        assert_eq!(binary_search_first(&arr, |&e| e <4), None);
    }

    #[test]
    fn first() {
        let arr = [0,1,2,3,4,5,6];
        //assert_eq!(binary_search_first(&arr, |&e| e <4), Some(3));
        assert_eq!(binary_search_first(&arr, |&e| e <=4), Some(4));
        assert_eq!(binary_search_first(&arr, |&e| e >4), Some(5));
        assert_eq!(binary_search_first(&arr, |&e| e >6), None);
    }
    */
}
