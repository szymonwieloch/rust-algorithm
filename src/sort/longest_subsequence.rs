/*!
Finds the longest sorted subsequence in the array.

**More:** <https://en.wikipedia.org/wiki/Longest_increasing_subsequence>


*/

use std::iter::repeat;
use std::iter::FromIterator;

fn longest_subsequence_impl(arr: &[i32]) -> Vec<usize>{
    if arr.is_empty() {
        return Vec::new();
    }
    let mut p: Vec<usize> = Vec::with_capacity(arr.len());
    let mut b: Vec<usize> = Vec::new();
    b.push(0);
    for i in 1..arr.len() {
        if arr[*b.last().unwrap()] < arr[i]
        {
            p[i] = *b.last().unwrap();
            b.push(i);
            continue;
        }
        // Binary search to find the smallest element referenced by b which is just bigger than a[i]
        // Note : Binary search is performed on b (and not a).
        // Size of b is always <=k and hence contributes O(log k) to complexity.
        let mut u = 0;
        let mut v = b.len()-1;
        while u<v {
            let c = (u+v)/2;
            if arr[b[c]] < arr[i] {
                u=c+1;
            } else {
                v=c;
            }
        }
        // Update b if new value is smaller then previously referenced value
        if arr[i] < arr[b[u]]
            {
                if u > 0 {
                    p[i] = b[u-1];
                }
                b[u] = i;
            }
    }
    b
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

pub fn longest_subsequence_asc<T>(arr: & [T],) -> usize
    where T: PartialOrd
{
    longest_subsequence_by(arr, |ref a, ref b| b>a)
}

pub fn longest_subsequence_wasc<T>(arr: & [T],) -> usize
    where T: PartialOrd
{
    longest_subsequence_by(arr, |ref a, ref b| b>=a)
}

pub fn longest_subsequence_desc<T>(arr: & [T],) -> usize
    where T: PartialOrd
{
    longest_subsequence_by(arr, |ref a, ref b| b<a)
}

pub fn longest_subsequence_wdesc<T>(arr: & [T],) -> usize
    where T: PartialOrd
{
    longest_subsequence_by(arr, |ref a, ref b| b<=a)
}