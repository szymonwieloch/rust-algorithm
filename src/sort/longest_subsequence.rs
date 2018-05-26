use search::binary_first_by;

fn is_ordered<'a, T>(a: &'a T, b: &'a T) -> bool where T: Ord {
    a<b
}

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