use utils::PairIterator;
use sort::Order;
use std::mem::swap;

#[inline(always)]
fn longest_ordered_substring_idx_impl<T, I, F>(iter: PairIterator<T, I>, mut is_ordered: F) -> (usize, usize)
    where
    T: Copy,
    I: Iterator<Item=T>,
    F: FnMut(T, T) -> bool
{
    let mut max_len: usize = 1;
    let mut max_idx: usize = 0;
    let mut curr_len = 1;
    let mut idx: usize = 0;
    for (a, b) in iter{
        if is_ordered(a, b){
            curr_len += 1;
        } else {
            if curr_len >  max_len{
                max_len = curr_len;
                max_idx = idx + 1 - curr_len;
            }
            curr_len = 1;
        }
        idx +=1;
    }
    if curr_len >  max_len{
        max_len = curr_len;
        max_idx = idx + 1 - curr_len;
    }
    (max_idx, max_idx+max_len)
}

#[inline(always)]
fn longest_ordered_substring_impl<T, I, F>(mut iter: I, mut is_ordered: F) -> Vec<T>
    where
        I: Iterator<Item=T>,
        F: FnMut(&T, &T) -> bool
{
    let prev = match iter.next() {
        None => return Vec::new(),
        Some(v) => v
    };
    let mut max_substr: Vec<T> = Vec::new();
    let mut curr_substr: Vec<T> = vec![prev];

    for curr in iter {
        if is_ordered(curr_substr.last().unwrap(), &curr){
            curr_substr.push(curr);
        } else {
            if curr_substr.len() >  max_substr.len(){
               swap(&mut max_substr, &mut curr_substr);
            }
            curr_substr.clear();
            curr_substr.push(curr);
        }
    }
    if curr_substr.len() >  max_substr.len() {
        curr_substr
    } else {
        max_substr
    }
}


/**
Finds index of the longest ordered substring in the provided collection using a custom comparator.

Returns starting and ending indexes of the substring.

**More:** <https://en.wikipedia.org/wiki/Substring>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::sort::longest_ordered_substring_idx_by;

fn main(){
    let arr = [1,2,3,2,1,2,3,4,5];
    //finds substring 1,2,3,4,5 = arr[4..9]
    assert_eq!(longest_ordered_substring_idx_by(&arr, |a, b| a<b), (4,9));
}
```
*/
pub fn longest_ordered_substring_idx_by<I, T, F>(iter: I, is_ordered: F) -> (usize, usize)
    where
        I: IntoIterator<Item = T>,
        F: FnMut(T, T) -> bool,
        T: Copy,
{
    let iter =  match PairIterator::try_new(iter){
        Some(i) => i,
        None => return (0, 0)
    };
    longest_ordered_substring_idx_impl(iter, is_ordered)
}


/**
Finds index of the longest ordered substring in the provided collection.

Returns starting and ending indexes of the substring.

**More:** <https://en.wikipedia.org/wiki/Substring>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::sort::longest_ordered_substring_idx;
use algorithm::sort::Order::*;

fn main(){
    let arr = [1,2,3,2,1,2,3,4,5];
    //finds substring 1,2,3,4,5 = arr[4..9]
    assert_eq!(longest_ordered_substring_idx(&arr, Increasing), (4,9));
}
```
*/
pub fn longest_ordered_substring_idx<I, T>(iter: I, order: Order) -> (usize, usize)
    where
        I: IntoIterator<Item = T>,
        T: Copy+PartialOrd,
{
    let iter =  match PairIterator::try_new(iter){
        Some(i) => i,
        None => return (0, 0)
    };
    match order {
        Order::Increasing => longest_ordered_substring_idx_impl(iter, |a,b| a<b),
        Order::Decreasing => longest_ordered_substring_idx_impl(iter, |a,b| a>b),
        Order::NotIncreasing => longest_ordered_substring_idx_impl(iter, |a,b| a>=b),
        Order::NotDecreasing => longest_ordered_substring_idx_impl(iter, |a,b| a<=b),
    }

}


/**
Finds of the longest ordered substring in the provided collection using a custom comparator.

Returns the substring in a form of a vector.

**More:** <https://en.wikipedia.org/wiki/Substring>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(k)

where k - length of the longest substring

# Example
```
extern crate algorithm;
use algorithm::sort::longest_ordered_substring_by;

fn main(){
    let v = vec![1,2,3,2,1,2,3,4,5];
    //finds substring 1,2,3,4,5
    assert_eq!(longest_ordered_substring_by(v, |a, b| a<b), vec![1,2,3,4,5]);
}
```
*/
pub fn longest_ordered_substring_by<I, T, F>(iter:I, is_ordered: F) -> Vec<T>
where
    I: IntoIterator<Item=T>,
    F: FnMut(&T, &T) -> bool
{
    longest_ordered_substring_impl(iter.into_iter(), is_ordered)
}


/**
Finds of the longest ordered substring in the provided collection.

Returns the substring in a form of a vector.

**More:** <https://en.wikipedia.org/wiki/Substring>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(k)

where k - length of the longest substring

# Example
```
extern crate algorithm;
use algorithm::sort::longest_ordered_substring;
use algorithm::sort::Order::*;

fn main(){
    let v = vec![1,2,3,2,1,2,3,4,5];
    //finds substring 1,2,3,4,5
    assert_eq!(longest_ordered_substring(v, Increasing), vec![1,2,3,4,5]);
}
```
*/
pub fn longest_ordered_substring<I, T>(iter:I, order: Order) -> Vec<T>
    where
        I: IntoIterator<Item=T>,
        T: PartialOrd
{
    let iter = iter.into_iter();
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
    fn idx_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(longest_ordered_substring_idx(&arr, Increasing), (0,0))
    }

    #[test]
    fn idx_single() {
        let arr = [5];
        assert_eq!(longest_ordered_substring_idx(&arr, Increasing), (0,1))
    }

    #[test]
    fn idx_decr() {
        let arr = [5,4,3,2,1];
        assert_eq!(longest_ordered_substring_idx(&arr, Increasing), (0,1))
    }

    #[test]
    fn idx_start() {
        let arr = [1,2,3,4,5,4];
        assert_eq!(longest_ordered_substring_idx(&arr, Increasing), (0,5))
    }

    #[test]
    fn idx_middle() {
        let arr = [5,4,3,2,3,4,5,2,1];
        assert_eq!(longest_ordered_substring_idx(&arr, Increasing), (3,7))
    }

    #[test]
    fn idx_end() {
        let arr = [5,4,3,2,1,0,1,2,3,4];
        assert_eq!(longest_ordered_substring_idx(&arr, Increasing), (5,10))
    }

    #[test]
    fn idx_two_equal() {
        let arr = [5,4,3,4,5,4,3,4,5,4,3,2,1];
        assert_eq!(longest_ordered_substring_idx(&arr, Increasing), (2,5))
    }

    #[test]
    fn idx_weak_grow() {
        let arr = [1,2,3,3,4,5,6];
        assert_eq!(longest_ordered_substring_idx(&arr, Increasing), (3,7))
    }

    #[test]
    fn empty() {
        let arr: [i32; 0] = [];
        assert!(longest_ordered_substring(&arr, Increasing).is_empty())
    }

    #[test]
    fn single() {
        assert_eq!(longest_ordered_substring(vec![5], Increasing), vec![5])
    }

    #[test]
    fn decr() {
        assert_eq!(longest_ordered_substring(vec![5,4,3,2,1], Increasing), vec![5])
    }

    #[test]
    fn start() {
        assert_eq!(longest_ordered_substring(vec![1,2,3,4,5,4], Increasing), vec![1,2,3,4,5])
    }

    #[test]
    fn middle() {
        assert_eq!(longest_ordered_substring(vec![5,4,3,2,3,4,5,2,1], Increasing), vec![2,3,4,5])
    }

    #[test]
    fn end() {
        assert_eq!(longest_ordered_substring(vec![5,4,3,2,1,0,1,2,3,4], Increasing), vec![0,1,2,3,4])
    }

    #[test]
    fn two_equal() {
        assert_eq!(longest_ordered_substring(vec![5,4,3,4,5,4,3,4,6,4,3,2,1], Increasing), vec![3,4,5])
    }

    #[test]
    fn weak_grow() {
        assert_eq!(longest_ordered_substring(vec![1,2,3,3,4,5,6], Increasing), vec![3,4,5,6])
    }


}