/**
Finds the longest ordered substring using the provided comparator.

Returns starting index of the substring and its length.

**More:** <https://en.wikipedia.org/wiki/Substring>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::sort::longest_substring_by;

fn main(){
    let arr = [1,2,3,2,1,2,3,4,5];
    //finds substring 1,2,3,4,5
    assert_eq!(longest_substring_by(&arr, |&a, &b| b>a), (4,5));
}
```
*/
pub fn longest_substring_by<'a, I, T, F>(iter: I, mut is_ordered: F) -> (usize, usize)
    where
        I: IntoIterator<Item = &'a T>,
        F: FnMut(&T, &T) -> bool,
        T: 'a,
{
    let mut it = iter.into_iter();
    let mut prev = match it.next() {
        Option::None => return (0, 0),
        Option::Some(first) => first,
    };
    let mut max_len: usize = 1;
    let mut max_idx: usize = 0;
    let mut curr_len = 1;
    for (idx, curr) in it.enumerate() {
        if is_ordered(prev, curr){
            curr_len += 1;
            if curr_len >  max_len{
                max_len = curr_len;
                max_idx = idx + 2 - curr_len;
            }
        } else {
            curr_len = 1;
        }
        prev = curr;
    }
    (max_idx, max_len)
}
/**
Finds the longest ascending substring.

Returns starting index of the substring and its length.

**More:** <https://en.wikipedia.org/wiki/Substring>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::sort::longest_substring_asc;

fn main(){
    let arr = [1,2,3,2,1,2,3,4,5];
    //finds substring 1,2,3,4,5
    assert_eq!(longest_substring_asc(&arr), (4,5));
}
```
*/
pub fn longest_substring_asc<'a, I, T>(iter: I) -> (usize, usize)
    where
        I: IntoIterator<Item = &'a T>,
        T: 'a+ PartialOrd,
{
    longest_substring_by(iter, |ref a, ref b| b>a)
}

/**
Finds the longest weakly ascending substring.

Returns starting index of the substring and its length.

**More:** <https://en.wikipedia.org/wiki/Substring>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::sort::longest_substring_wasc;

fn main(){
    let arr = [1,2,3,2,1,2,3,3,4];
    //finds substring 1,2,3,3,4
    assert_eq!(longest_substring_wasc(&arr), (4,5));
}
```
*/
pub fn longest_substring_wasc<'a, I, T>(iter: I) -> (usize, usize)
    where
        I: IntoIterator<Item = &'a T>,
        T: 'a+ PartialOrd,
{
    longest_substring_by(iter, |ref a, ref b| b>=a)
}

/**
Finds the longest descending substring.

Returns starting index of the substring and its length.

**More:** <https://en.wikipedia.org/wiki/Substring>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::sort::longest_substring_desc;

fn main(){
    let arr = [1,2,3,4,5,4,3,2,1];
    //finds substring 5,4,3,2,1
    assert_eq!(longest_substring_desc(&arr), (4,5));
}
```
*/
pub fn longest_substring_desc<'a, I, T>(iter: I) -> (usize, usize)
    where
        I: IntoIterator<Item = &'a T>,
        T: 'a+ PartialOrd,
{
    longest_substring_by(iter, |ref a, ref b| b<a)
}

/**
Finds the longest descending substring.

Returns starting index of the substring and its length.

**More:** <https://en.wikipedia.org/wiki/Substring>

# Complexity

- Processing complexity: O(n)
- Memory complexity: O(1)

# Example
```
extern crate algorithm;
use algorithm::sort::longest_substring_wdesc;

fn main(){
    let arr = [1,2,3,4,5,4,3,2,2];
    //finds substring 5,4,3,2,2
    assert_eq!(longest_substring_wdesc(&arr), (4,5));
}
```
*/
pub fn longest_substring_wdesc<'a, I, T>(iter: I) -> (usize, usize)
    where
        I: IntoIterator<Item = &'a T>,
        T: 'a+ PartialOrd,
{
    longest_substring_by(iter, |ref a, ref b| b<=a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let arr: [i32; 0] = [];
        assert_eq!(longest_substring_asc(&arr), (0,0))
    }

    #[test]
    fn single() {
        let arr = [5];
        assert_eq!(longest_substring_asc(&arr), (0,1))
    }

    #[test]
    fn decr() {
        let arr = [5,4,3,2,1];
        assert_eq!(longest_substring_asc(&arr), (0,1))
    }

    #[test]
    fn start() {
        let arr = [1,2,3,4,5,4];
        assert_eq!(longest_substring_asc(&arr), (0,5))
    }

    #[test]
    fn middle() {
        let arr = [5,4,3,2,3,4,5,2,1];
        assert_eq!(longest_substring_asc(&arr), (3,4))
    }

    #[test]
    fn end() {
        let arr = [5,4,3,2,1,0,1,2,3,4];
        assert_eq!(longest_substring_asc(&arr), (5,5))
    }

    #[test]
    fn two_equal() {
        let arr = [5,4,3,4,5,4,3,4,5,4,3,2,1];
        assert_eq!(longest_substring_asc(&arr), (2,3))
    }

    #[test]
    fn weak_grow() {
        let arr = [1,2,3,3,4,5,6];
        assert_eq!(longest_substring_asc(&arr), (3,4))
    }
}