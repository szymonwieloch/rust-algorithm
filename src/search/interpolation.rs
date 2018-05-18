use std::cmp::Ordering;
/**

**More:** <https://en.wikipedia.org/wiki/Interpolation_search>

*/
pub fn interpolation_search_asc<'a, T>(arr: &'a [T], val: T) -> Option<usize>
where T: Ord + Copy,
 f64: From<T>
{
    if arr.is_empty(){
        return None;
    }
    let mut lo = 0usize;
    let mut hi = arr.len() -1;
    while lo <= hi && val >= arr[lo] && val <= arr[hi]
    {
        // Probing the position with keeping
        // uniform distribution in mind.
        let y_lo = f64::from(arr[lo]);
        let x_diff = (hi-lo) as f64;
        let y_diff = f64::from(arr[hi]) - y_lo;
        let val_diff: f64 = y_lo - f64::from(val);

        let mid =  lo + ((x_diff *(val_diff/y_diff)) as usize);

        match val.cmp(&arr[mid]) {
            Ordering::Less => hi = mid -1,
            Ordering::Greater => lo = mid+1,
            Ordering::Equal => return Option::Some(mid as usize)
        }
    }
    Option::None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asc_empty() {
        let arr: [i32; 0] = [];
        assert!(interpolation_search_asc(&arr, 7).is_none());
    }

    #[test]
    fn asc_one() {
        let arr = [5];
        assert!(interpolation_search_asc(&arr, 7).is_none());
    }

    #[test]
    fn asc_found() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(interpolation_search_asc(&arr, 12), Some(3));
    }
    #[test]
    fn asc_too_small() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(interpolation_search_asc(&arr, 3), None);
    }

    #[test]
    fn asc_not_found() {
        let arr = [5, 7, 8, 12, 22, 33];
        assert_eq!(interpolation_search_asc(&arr, 13), None);
    }
}