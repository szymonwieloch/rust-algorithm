use rand::{thread_rng, Rng};

#[inline]
pub fn partition<T, F>(arr: &mut [T], mut is_ordered: F) -> usize
    where
        F: FnMut(&T, &T) -> bool
{
    //use the last element for comparison
    let last_idx = arr.len() - 1;
    let mut store_idx = 0;
    for i in 0..last_idx {
        if is_ordered(unsafe { arr.get_unchecked(i) },  unsafe { arr.get_unchecked(last_idx) }) {
            arr.swap(store_idx, i);
            store_idx += 1;
        }
    }
    arr.swap(store_idx, last_idx);
    store_idx
}

#[inline]
pub fn partition_rand<T, F>(arr: &mut [T], mut is_ordered: F) -> usize
    where
        F: FnMut(&T, &T) -> bool
{
    let pivot_idx = thread_rng().gen_range(0, arr.len());
    let last_idx = arr.len() - 1;
    arr.swap(pivot_idx, last_idx);
    let mut store_idx = 0;
    for i in 0..last_idx {
        if is_ordered(unsafe { arr.get_unchecked(i) }, unsafe{arr.get_unchecked(last_idx)}) {
            arr.swap(store_idx, i);
            store_idx += 1;
        }
    }
    arr.swap(store_idx, last_idx);
    store_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_simple() {
        let mut arr = [9, 4, 8, 3, 7];
        let idx = partition(&mut arr, |a,b| a<b);
        assert_eq!(arr, [4, 3, 7, 9, 8]);
        assert_eq!(idx, 2)
    }

    #[test]
    fn partition_complex() {
        let mut arr = [8, 4, 1, 2, 8, 9, 4, 6, 6, 5];
        let idx = partition(&mut arr, |a,b| a<b);
        assert_eq!(arr, [4, 1, 2, 4, 5, 9, 8, 6, 6, 8]);
        assert_eq!(idx, 4)
    }

    #[test]
    fn partition_first() {
        let mut arr = [6, 5, 8, 1];
        let idx = partition(&mut arr, |a,b| a<b);
        assert_eq!(arr, [1, 5, 8, 6]);
        assert_eq!(idx, 0)
    }

    #[test]
    fn partition_last() {
        let mut arr = [3, 4, 8, 3, 9];
        let idx = partition(&mut arr, |a,b| a<b);
        assert_eq!(arr, [3, 4, 8, 3, 9]);
        assert_eq!(idx, 4)
    }
}