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