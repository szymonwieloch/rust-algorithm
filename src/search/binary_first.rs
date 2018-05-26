//this is the actual implementation of binary search
//it is inlined so that the comparator gets inlined too
#[inline(always)]
fn binary_first_impl<'a, T, F>(arr: &'a [T], mut to_left: F) -> Option<usize>
    where
        F: FnMut(&'a T) -> bool,
{
    //This check is needed because otherwise hi can be lower than 0
    if arr.is_empty(){
        return None;
    }
    //highest and lowest indexes of the subslice being checked
    let mut lo: usize = 0;
    let mut hi: usize = arr.len() -1;
    while lo < hi {
        //it is better to use (hi-lo)/2 because writing (hi+2)/2 can lead to a overflow
        let mid = lo + (hi-lo)/2;
        //this i safe, mid is always in the range [lo..hi]
        if to_left(unsafe{arr.get_unchecked(mid)}) {
            if mid == 0{
                //prevent underflow
                hi = 0;
            } else {
                hi = mid - 1;
            }
        } else {
            lo = mid + 1;
        }
    }
    //now lo == hi, this is the found element
    //but still the condition might be met or not
    let first = if to_left(unsafe{arr.get_unchecked(lo)}) {
        lo
    } else {
        lo +1
    };
    //first may be outside of slice if no element meets criteria
    if first == arr.len(){
        None
    } else {
        Some(first)
    }
}

/**
Using binary search finds the first element in the slice that meets the given criteria.

Classic binary search can only find an element that is **equal** to the provided value.
This algorithms allows searching for elements that are for example greater than the given value.

It is assumed that elements meeting the given criteria are at the end of the slice and
elements not meeting the given criteria are at the beginning of the slice (this refers to
sorting order). This function find index of the first element that meets the given criteria.

# Example
```
extern crate algorithm;
use algorithm::search::binary_first_by;
fn main(){
    let letters = ['a', 'b', 'c', 'd', 'e', 'f'];
    //find index of the first element that is greater than 'd'
    assert_eq!(binary_first_by(&letters, |x|*x>'d'), Some(4));

    let numbers = [50, 40, 30, 20, 10, 0];
    //find index of the first element that is smaller or equal to 20
    assert_eq!(binary_first_by(&numbers, |x|*x<=20), Some(3));
}
```
*/
pub fn binary_first_by<'a, T, F>(arr: &'a [T], to_left: F) -> Option<usize>
where
F: FnMut(&'a T) -> bool
{
    binary_first_impl(arr, to_left)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty(){
        let arr: [i32;0] = [];
        assert_eq!(binary_first_by(&arr, |a|*a>5), None);
    }

    #[test]
    fn single_found(){
        let arr = [5];
        assert_eq!(binary_first_by(&arr, |a|*a>4), Some(0));
    }

    #[test]
    fn single_not_found(){
        let arr = [5];
        assert_eq!(binary_first_by(&arr, |a|*a>6), None);
    }

    #[test]
    fn incr_gr_found(){
        let arr = [0,1,2,3,4,5];
        assert_eq!(binary_first_by(&arr, |a|*a>3), Some(4));
    }

    #[test]
    fn incr_ge_found(){
        let arr = [0,1,2,3,4,5];
        assert_eq!(binary_first_by(&arr, |a|*a>=3), Some(3));
    }

    #[test]
    fn incr_not_found(){
        let arr = [0,1,2,3,4,5];
        assert_eq!(binary_first_by(&arr, |a|*a>=6), None);
    }

    #[test]
    fn incr_last(){
        let arr = [0,1,2,3,4,5];
        assert_eq!(binary_first_by(&arr, |a|*a>=5), Some(5));
    }

    #[test]
    fn incr_first(){
        let arr = [0,1,2,3,4,5];
        assert_eq!(binary_first_by(&arr, |a|*a>=0), Some(0));
    }

    #[test]
    fn decr_gr_found(){
        let arr = [5,4,3,2,1,0];
        assert_eq!(binary_first_by(&arr, |a|*a<3), Some(3));
    }

    #[test]
    fn decr_ge_found(){
        let arr = [5,4,3,2,1,0];
        assert_eq!(binary_first_by(&arr, |a|*a<=3), Some(2));
    }

    #[test]
    fn decr_not_found(){
        let arr = [5,4,3,2,1,0];
        assert_eq!(binary_first_by(&arr, |a|*a<0), None);
    }

    #[test]
    fn decr_first(){
        let arr = [5,4,3,2,1,0];
        assert_eq!(binary_first_by(&arr, |a|*a<=5), Some(0));
    }

    #[test]
    fn decr_last(){
        let arr = [5,4,3,2,1,0];
        assert_eq!(binary_first_by(&arr, |a|*a<=0), Some(5));
    }
}