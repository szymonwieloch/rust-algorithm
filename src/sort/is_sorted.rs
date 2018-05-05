use std::iter::Iterator;

pub fn is_increasing<I, T>(mut it: I) -> bool where I: Iterator<Item=T>, T:PartialOrd{
    let mut prev = match it.next() {
        Option::None => return true,
        Option::Some(first) => first
    };
    for curr in it {
        if curr <= prev {
            return false;
        }
        prev = curr;
    }
    return true;
}

pub fn is_decreasing<I, T>(mut it: I) -> bool where I: Iterator<Item=T>, T:PartialOrd{
    let mut prev = match it.next() {
        Option::None => return true,
        Option::Some(first) => first
    };
    for curr in it {
        if curr >= prev {
            return false;
        }
        prev = curr;
    }
    return true;
}

pub fn is_not_increasing<I, T>(mut it: I) -> bool where I: Iterator<Item=T>, T:PartialOrd{
    let mut prev = match it.next() {
        Option::None => return true,
        Option::Some(first) => first
    };
    for curr in it {
        if curr > prev {
            return false;
        }
        prev = curr;
    }
    return true;
}

pub fn is_not_decreasing<I, T>(mut it: I) -> bool where I: Iterator<Item=T>, T:PartialOrd{
    let mut prev = match it.next() {
        Option::None => return true,
        Option::Some(first) => first
    };
    for curr in it {
        if curr < prev {
            return false;
        }
        prev = curr;
    }
    return true;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increase_empty() {
        let arr :[i32;0] = [];
        assert!(is_increasing(arr.iter()));
    }

    #[test]
    fn increase_single() {
        let arr = [1];
        assert!(is_increasing(arr.iter()));
    }

    #[test]
    fn increase_true() {
        let arr = [1, 4, 6, 10];
        assert!(is_increasing(arr.iter()));
    }

    #[test]
    fn increase_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_increasing(arr.iter()));
    }

    #[test]
    fn increase_equal() {
        let arr = [1, 4, 6, 6, 10];
        assert!(!is_increasing(arr.iter()));
    }

    #[test]
    fn decrease_empty() {
        let arr :[i32;0] = [];
        assert!(is_decreasing(arr.iter()));
    }

    #[test]
    fn decrease_single() {
        let arr = [1];
        assert!(is_decreasing(arr.iter()));
    }

    #[test]
    fn decrease_true() {
        let arr = [55, 44, 33, 22, 11];
        assert!(is_decreasing(arr.iter()));
    }

    #[test]
    fn decrease_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_decreasing(arr.iter()));
    }

    #[test]
    fn decrease_equal() {
        let arr = [19, 9, 6, 6, 1];
        assert!(!is_decreasing(arr.iter()));
    }

    #[test]
    fn no_increase_empty() {
        let arr :[i32;0] = [];
        assert!(is_not_increasing(arr.iter()));
    }

    #[test]
    fn no_increase_single() {
        let arr = [1];
        assert!(is_not_increasing(arr.iter()));
    }

    #[test]
    fn no_increase_true() {
        let arr = [44, 33, 22, 11, 0];
        assert!(is_not_increasing(arr.iter()));
    }

    #[test]
    fn no_increase_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_not_increasing(arr.iter()));
    }

    #[test]
    fn no_increase_equal() {
        let arr = [33, 23, 23, 5, 4, 4, 1];
        assert!(is_not_increasing(arr.iter()));
    }

    #[test]
    fn no_decrease_empty() {
        let arr :[i32;0] = [];
        assert!(is_not_decreasing(arr.iter()));
    }

    #[test]
    fn no_decrease_single() {
        let arr = [1];
        assert!(is_not_decreasing(arr.iter()));
    }

    #[test]
    fn no_decrease_true() {
        let arr = [1, 6, 9, 10];
        assert!(is_not_decreasing(arr.iter()));
    }

    #[test]
    fn no_decrease_false() {
        let arr = [1, 4, 6, 5];
        assert!(!is_not_decreasing(arr.iter()));
    }

    #[test]
    fn no_decrease_equal() {
        let arr = [4, 6, 8, 8, 9, 10, 10];
        assert!(is_not_decreasing(arr.iter()));
    }
}