use std::cmp::Ordering;

pub fn binary_asc<T>(arr: &[T], val: &T) -> Option<usize> where T: Ord {
    let mut lo: isize = 0;
    let mut hi: isize = arr.len() as isize -1;
    while lo <= hi {
        let mid = lo + (hi-lo)/2;
        match unsafe{arr.get_unchecked(mid as usize)}.cmp(val) {
            Ordering::Greater => hi = mid -1,
            Ordering::Less => lo = mid+1,
            Ordering::Equal => return Option::Some(mid as usize)
        }
    }
    Option::None
}

pub fn binary_desc<T>(arr: &[T], val: &T) -> Option<usize> where T: Ord {
    let mut lo: isize = 0;
    let mut hi: isize = arr.len() as isize -1;
    while lo <= hi {
        let mid = lo + (hi-lo)/2;
        match unsafe{arr.get_unchecked(mid as usize)}.cmp(val) {
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
        let arr : [i32; 0] = [];
        assert!(binary_asc(&arr, &7).is_none());
    }

    #[test]
    fn asc_one() {
        let arr  = [5];
        assert!(binary_asc(&arr, &7).is_none());
    }

    #[test]
    fn asc_found() {
        let arr  = [5, 7, 8, 12, 22, 33];
        assert_eq!(binary_asc(&arr, &12), Some(3));
    }
    #[test]
    fn asc_too_small() {
        let arr  = [5, 7, 8, 12, 22, 33];
        assert_eq!(binary_asc(&arr, &3), None);
    }

    #[test]
    fn asc_not_found() {
        let arr  = [5, 7, 8, 12, 22, 33];
        assert_eq!(binary_asc(&arr, &13), None);
    }

    #[test]
    fn desc_empty() {
        let arr : [i32; 0] = [];
        assert!(binary_desc(&arr, &7).is_none());
    }

    #[test]
    fn desc_one() {
        let arr  = [5];
        assert!(binary_desc(&arr, &7).is_none());
    }

    #[test]
    fn desc_found() {
        let arr  = [44, 33, 22, 9, 7, 4, 2, 1];
        assert_eq!(binary_desc(&arr, &2), Some(6));
    }

    #[test]
    fn desc_not_found() {
        let arr  = [44, 33, 22, 9, 7, 4, 2, 1];
        assert_eq!(binary_asc(&arr, &66), None);
    }
}