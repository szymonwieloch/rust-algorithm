use std::iter::repeat;
use std::iter::FromIterator;
use search::binary_first_by;

fn longest_subsequence<T>(arr: &[T]) -> Vec<usize>
    where
        T: Ord,
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
        if arr[*result.last().unwrap()] < arr[i] {
            previous_chain[i] = *result.last().unwrap();
            result.push(i);
            continue;
        }

        // Perform a binary search to find the index of an item in `result` to overwrite.
        // We want to overwrite an index that refers to the smallest item that is larger than `items[i]`.
        // If there is no such item, then we do nothing.
        let comparator = |&result_index| {
            use std::cmp::Ordering;

            // We don't return Ordering::Equal when we find an equal value,
            // because we want to find the index of the first equal value.
            if arr[result_index] < arr[i]{
                Ordering::Less
            } else {
                Ordering::Greater
            }
        };

        let next_element_index = match result.binary_search_by(comparator) {
            Ok(index) | Err(index) => index,
        };

        if arr[i] < arr[result[next_element_index]] {
            if next_element_index > 0 {
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

pub fn longest_subsequence_by<T, F>(arr: & [T], mut is_ordered: F) -> usize
    where F: FnMut(&T, &T) -> bool
{
    let mut lis: Vec<usize> = Vec::from_iter(repeat(1 as usize).take(arr.len()));
    for i in 1..arr.len(){
        for j in 0..i {
            if lis[i] <= lis[j] && is_ordered(&arr[j], &arr[i]) {
                lis[i] = lis[j] + 1;
            }
        }
    }
    match lis.iter().max(){
        Option::None => 0,
        Option::Some(m) => *m
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use super::Order::*;

    #[test]
    fn empty() {
        let arr: [i32;0] = [];
        assert!(longest_subsequence(&arr).is_empty());
    }

    #[test]
    fn single() {
        let arr  = [5];
        assert_eq!(longest_subsequence(&arr), vec![0]);
    }

    #[test]
    fn simple() {
        let arr  = [2,3,0,5];
        assert_eq!(longest_subsequence(&arr), vec![0,1,3]);
    }

    #[test]
    fn advanced() {
        let arr  = [3,4,-1,5,8,2,3,12,7,9,10];
        assert_eq!(longest_subsequence(&arr), vec![2,5,6,8,9,10]);
    }

    #[test]
    fn decr() {
        let arr  = [5,4,3,2,1];
        assert_eq!(longest_subsequence(&arr), vec![4]);
    }

    #[test]
    fn first_mid_last() {
        let arr  = [5,10,6,3,7];
        assert_eq!(longest_subsequence(&arr), vec![0,2,4]);
    }
}