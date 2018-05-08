use super::super::search::{quick_select, quick_select_rand};
use std::ops::{Add, Div};

#[inline]
fn check_len(len: usize){
    if len == 0{
        panic!("Cannot calculate median of empty collection");
    }
}

pub fn median<T>(arr: &mut [T]) -> T where T: Clone+PartialOrd {
    let len = arr.len();
    check_len(len);
    quick_select(arr, len/2)
}

pub fn median_rand<T>(arr: &mut [T]) -> T where T: Clone+PartialOrd {
    let len = arr.len();
    check_len(len);
    quick_select_rand(arr, len/2)
}

/**

**More:** <https://en.wikipedia.org/wiki/Median>
*/

pub fn median_avg<T>(arr: &mut [T]) -> T where T: Clone+Ord+Add<Output=T>+Div<Output=T>+From<i32> {
    median_avg_impl(arr, quick_select)
}

pub fn median_avg_rand<T>(arr: &mut [T]) -> T where T: Clone+Ord+Add<Output=T>+Div<Output=T>+From<i32> {
    median_avg_impl(arr, quick_select_rand)
}


fn median_avg_impl<T>(arr: &mut [T], qs: fn(&mut [T], usize)->T) -> T where T: Clone+Ord+Add<Output=T>+Div<Output=T>+From<i32>{
    let len = arr.len();
    check_len(len);
    let mid = qs(arr, len/2);
    if len%2 == 1{
        mid
    } else {
        //after quick_select the array is partially sorted
        //all smaller elements are on the left, all bigger on the right
        let prev = arr[..len/2].iter().max().unwrap();
        (mid+prev.clone())/T::from(2)
    }
}