use utils::PairIterator;
use sort::Order;

#[inline(always)]
fn longest_ordered_substring_impl<T, I, F>(iter: PairIterator<T, I>, mut is_ordered: F) -> (usize, usize)
    where
    T: Copy,
    I: Iterator<Item=T>,
    F: FnMut(T, T) -> bool
{
    let mut max_len: usize = 1;
    let mut max_idx: usize = 0;
    let mut curr_len = 1;
    for (idx, (a, b)) in iter.enumerate(){
        if is_ordered(a, b){
            curr_len += 1;
            if curr_len >  max_len{
                max_len = curr_len;
                max_idx = idx + 2 - curr_len;
            }
        } else {
            curr_len = 1;
        }
    }
    (max_idx, max_len)
}


/**
Finds the longest ordered substring in the provided collection using a custom comparator.

Returns starting index of the substring and its length.

**More:** <https://en.wikipedia.org/wiki/Substring>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::sort::longest_ordered_substring_by;

fn main(){
    let arr = [1,2,3,2,1,2,3,4,5];
    //finds substring 1,2,3,4,5
    assert_eq!(longest_ordered_substring_by(&arr, |a, b| a<b), (4,5));
}
```
*/
pub fn longest_ordered_substring_by<I, T, F>(iter: I, is_ordered: F) -> (usize, usize)
    where
        I: IntoIterator<Item = T>,
        F: FnMut(T, T) -> bool,
        T: Copy,
{
    let iter =  match PairIterator::try_new(iter){
        Some(i) => i,
        None => return (0, 0)
    };
    longest_ordered_substring_impl(iter, is_ordered)
}


/**
Finds the longest ordered substring in the provided collection.

Returns starting index of the substring and its length.

**More:** <https://en.wikipedia.org/wiki/Substring>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::sort::longest_ordered_substring;
use algorithm::sort::Order::*;

fn main(){
    let arr = [1,2,3,2,1,2,3,4,5];
    //finds substring 1,2,3,4,5
    assert_eq!(longest_ordered_substring(&arr, Increasing), (4,5));
}
```
*/
pub fn longest_ordered_substring<I, T>(iter: I, order: Order) -> (usize, usize)
    where
        I: IntoIterator<Item = T>,
        T: Copy+PartialOrd,
{
    let iter =  match PairIterator::try_new(iter){
        Some(i) => i,
        None => return (0, 0)
    };
    match order {
        Order::Increasing => longest_ordered_substring_impl(iter, |a,b| a<b),
        Order::Decreasing => longest_ordered_substring_impl(iter, |a,b| a>b),
        Order::NotIncreasing => longest_ordered_substring_impl(iter, |a,b| a>=b),
        Order::NotDecreasing => longest_ordered_substring_impl(iter, |a,b| a<=b),
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Order::*;

    #[test]
    fn empty() {
        let arr: [i32; 0] = [];
        assert_eq!(longest_ordered_substring(&arr, Increasing), (0,0))
    }

    #[test]
    fn single() {
        let arr = [5];
        assert_eq!(longest_ordered_substring(&arr, Increasing), (0,1))
    }

    #[test]
    fn decr() {
        let arr = [5,4,3,2,1];
        assert_eq!(longest_ordered_substring(&arr, Increasing), (0,1))
    }

    #[test]
    fn start() {
        let arr = [1,2,3,4,5,4];
        assert_eq!(longest_ordered_substring(&arr, Increasing), (0,5))
    }

    #[test]
    fn middle() {
        let arr = [5,4,3,2,3,4,5,2,1];
        assert_eq!(longest_ordered_substring(&arr, Increasing), (3,4))
    }

    #[test]
    fn end() {
        let arr = [5,4,3,2,1,0,1,2,3,4];
        assert_eq!(longest_ordered_substring(&arr, Increasing), (5,5))
    }

    #[test]
    fn two_equal() {
        let arr = [5,4,3,4,5,4,3,4,5,4,3,2,1];
        assert_eq!(longest_ordered_substring(&arr, Increasing), (2,3))
    }

    #[test]
    fn weak_grow() {
        let arr = [1,2,3,3,4,5,6];
        assert_eq!(longest_ordered_substring(&arr, Increasing), (3,4))
    }
}