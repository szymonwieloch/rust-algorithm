use rand::{Rng, thread_rng};


/**
    Shuffles elements in the slice.

    Shuffling is the process of creating a random order in the slice.
    Is is therefore the opposite of sorting.

    # Complexity
    - Processing complexity: O(n)
    - Memory complexity: O(1)

    # Example
    ```
    extern crate algorithm;
    use algorithm::sort::shuffle;

    fn main(){
        let mut arr = [1,2,3,4,5];
        shuffle(&mut arr);
        //now elements in arr are randomly ordered
    }
    ```
*/
pub fn shuffle<T>(arr: &mut [T]) {
    //it turns out that rand already has an efficient implementation, so just forward this call
    //so that users of this crate have access to all functions without the need for external sources
    thread_rng().shuffle(arr)
}