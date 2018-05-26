use search::binary_first_by;
use sort::Order;

/**
Find indexes of the longest ordered subsequence in the slice using custom comparator.

**More:** <https://en.wikipedia.org/wiki/Longest_increasing_subsequence>

# Complexity
- Processing complexity: O(n*log(n))
- Memory complexity: O(n)

# Example
```
extern crate algorithm;
use algorithm::sort::longest_ordered_subsequence_by;

fn main(){
     let arr  = [3,4,-1,5,8,2,3,12,7,9,10];
     //the longest increasing subsequence is [-1,2,3,7,9,10]
     //the function returns indexes:
     assert_eq!(longest_ordered_subsequence_by(&arr, |a,b| a<b), vec![2,5,6,8,9,10]);
}
```
*/
pub fn longest_ordered_subsequence_by<T, F>(arr: &[T], is_ordered:F) -> Vec<usize>
    where
        F: FnMut(&T, &T) -> bool
{
    longest_subsequence_impl(arr, is_ordered)
}


/**
Find indexes of the longest ordered subsequence in the slice.

**More:** <https://en.wikipedia.org/wiki/Longest_increasing_subsequence>

# Complexity
- Processing complexity: O(n*log(n))
- Memory complexity: O(n)

# Example
```
extern crate algorithm;
use algorithm::sort::longest_ordered_subsequence;
use algorithm::sort::Order::*;

fn main(){
     let arr  = [3,4,-1,5,8,2,3,12,7,9,10];
     //the longest increasing subsequence is [-1,2,3,7,9,10]
     //the function returns indexes:
     assert_eq!(longest_ordered_subsequence(&arr, Increasing), vec![2,5,6,8,9,10]);
}
```
*/
pub fn longest_ordered_subsequence<T>(arr: &[T], order: Order) -> Vec<usize>
    where
        T: Ord,
{
    match order{
        Order::Increasing => longest_subsequence_impl(arr, |a, b| a<b),
        Order::Decreasing => longest_subsequence_impl(arr, |a, b| a>b),
        Order::NotDecreasing => longest_subsequence_impl(arr, |a, b| a<=b),
        Order::NotIncreasing => longest_subsequence_impl(arr, |a, b| a>=b),
    }
}

#[inline(always)]
fn longest_subsequence_impl<T, F>(arr: &[T], mut is_ordered:F) -> Vec<usize>
    where
        F: FnMut(&T, &T) -> bool
{
    //adapted from https://codereview.stackexchange.com/questions/187337/longest-increasing-subsequence-algorithm

    let mut result = Vec::new();

    //if arr is empty, then the result is too
    if arr.is_empty() {
        return result;
    }

    // This vector stores, for each item,
    // the index of the largest item prior to itself that is smaller than itself.
    // We'll use this vector at the end to build the final result.
    let mut previous_chain = vec![0; arr.len()];

    // Initially, we assume that the first item is part of the result.
    // We will replace this index later if that's not the case.
    result.push(0);

    for i in 1..arr.len() {
        // If the next item is greater than the last item of the current longest subsequence,
        // push its index at the end of the result and continue.
        if is_ordered(&arr[*result.last().unwrap()], &arr[i]) {
            previous_chain[i] = *result.last().unwrap();
            result.push(i);
            continue;
        }

        // We want to overwrite an index that refers to the smallest item that is larger than arr[i].
        // If there is no such item, then we do nothing.
        if let Some(next_element_index) = binary_first_by(&result, |a |  is_ordered(&arr[i], &arr[*a]) ) {
            if next_element_index>0 {
                previous_chain[i] = result[next_element_index - 1];
            }
            result[next_element_index] = i;
        }


    }
    // The last item in `result` is correct,
    // but we might have started overwriting earlier items
    // with what could have been a longer subsequence.
    // Walk back `previous_chain` to restore the proper subsequence.
    let mut u = result.len();
    let mut v = *result.last().unwrap();
    while u != 0 {
        u -= 1;
        result[u] = v;
        v = previous_chain[v];
    }
    result
}



#[cfg(test)]
mod tests {
    use super::*;
    use super::Order::*;

    #[test]
    fn incr_empty() {
        let arr: [i32;0] = [];
        assert!(longest_ordered_subsequence(&arr, Increasing).is_empty());
    }

    #[test]
    fn incr_single() {
        let arr  = [5];
        assert_eq!(longest_ordered_subsequence(&arr, Increasing), vec![0]);
    }

    #[test]
    fn incr_simple() {
        let arr  = [2,3,0,5];
        assert_eq!(longest_ordered_subsequence(&arr, Increasing), vec![0, 1, 3]);
    }

    #[test]
    fn incr_advanced() {
        let arr  = [3,4,-1,5,8,2,3,12,7,9,10];
        assert_eq!(longest_ordered_subsequence(&arr, Increasing), vec![2, 5, 6, 8, 9, 10]);
    }

    #[test]
    fn incr_decr() {
        let arr  = [5,4,3,2,1];
        assert_eq!(longest_ordered_subsequence(&arr, Increasing), vec![4]);
    }

    #[test]
    fn incr_first_mid_last() {
        let arr  = [5,10,6,3,7];
        assert_eq!(longest_ordered_subsequence(&arr, Increasing), vec![0, 2, 4]);
    }

    #[test]
    fn decr_empty() {
        let arr: [i32;0] = [];
        assert!(longest_ordered_subsequence(&arr, Decreasing).is_empty());
    }

    #[test]
    fn decr_single() {
        let arr  = [5];
        assert_eq!(longest_ordered_subsequence(&arr, Decreasing), vec![0]);
    }

    #[test]
    fn decr_simple() {
        let arr  = [5,4,6,3];
        assert_eq!(longest_ordered_subsequence(&arr, Decreasing), vec![0, 1, 3]);
    }

    #[test]
    fn decr_advanced() {
        let arr  = [-3,-4,1,-5,-8,-2,-3,-12,-7,-9,-10];
        assert_eq!(longest_ordered_subsequence(&arr, Decreasing), vec![2, 5, 6, 8, 9, 10]);
    }

    #[test]
    fn decr_incr() {
        let arr  = [1,2,3,4,5];
        assert_eq!(longest_ordered_subsequence(&arr, Decreasing), vec![4]);
    }

    #[test]
    fn decr_first_mid_last() {
        let arr = [-5, -10, -6, -3, -7];
        assert_eq!(longest_ordered_subsequence(&arr, Decreasing), vec![0, 2, 4]);
    }

    #[test]
    fn nincr_simple() {
        let arr  = [5,4,4,6,3];
        assert_eq!(longest_ordered_subsequence(&arr, NotIncreasing), vec![0, 1, 2, 4]);
    }

    #[test]
    fn nincr_advanced() {
        let arr  = [10,6,9,2,7,4,1,3,3,1];
        assert_eq!(longest_ordered_subsequence(&arr, NotIncreasing), vec![0, 2, 4, 5, 7, 8, 9]);
    }

}