use std::iter::repeat;
use std::iter::FromIterator;

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