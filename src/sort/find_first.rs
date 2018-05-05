

pub fn first_not_increasing<I, T>(mut it: I) -> Option<usize> where I: Iterator<Item=T>, T:PartialOrd {
    let mut prev = match it.next() {
        Option::None => return None,
        Option::Some(first) => first
    };
    for (i, curr) in it.enumerate() {
        if curr <= prev {
            return Some(i+1);
        }
        prev = curr;
    }
    return None;
}

pub fn first_not_decreasing<I, T>(mut it: I) -> Option<usize> where I: Iterator<Item=T>, T:PartialOrd {
    let mut prev = match it.next() {
        Option::None => return None,
        Option::Some(first) => first
    };
    for (i, curr) in it.enumerate() {
        if curr >= prev {
            return Some(i+1);
        }
        prev = curr;
    }
    return None;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_increase_empty() {
        let arr: [i32; 0] = [];
        assert!(first_not_increasing(arr.iter()).is_none());
    }

    #[test]
    fn no_increase_single() {
        let arr = [5];
        assert!(first_not_increasing(arr.iter()).is_none());
    }

    #[test]
    fn no_increase_found() {
        let arr = [5, 7, 8, 10, 4, 11, 12];
        assert_eq!(first_not_increasing(arr.iter()), Some(4));
    }

    #[test]
    fn no_decrease_empty() {
        let arr: [i32; 0] = [];
        assert!(first_not_decreasing(arr.iter()).is_none());
    }

    #[test]
    fn no_decrease_single() {
        let arr = [5];
        assert!(first_not_decreasing(arr.iter()).is_none());
    }

    #[test]
    fn no_decrease_found() {
        let arr = [44, 33, 22, 88, 11];
        assert_eq!(first_not_decreasing(arr.iter()), Some(3));
    }
}