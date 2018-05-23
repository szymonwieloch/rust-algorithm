use std::iter::{IntoIterator, Iterator};
use std::convert::From;
use std::mem::uninitialized;

/**
Iterator over pairs in the given collection.

This is very useful for working on relationships between elements in the collection,
for example finding first increasing pair, checking if the collection is sorted etc.

# Example
```
extern crate algorithm;
use algorithm::utils::PairIterator;
use std::convert::From;

fn main() {
    //find first decreasing element;
    let decr = [1,2,3,4,5,6,7,0];
    let mut iter = PairIterator::from(&decr);
    assert_eq!(iter.position(|(a,b)|a>b), Some(6));

    //check if the collection is sorted
    let incr = [1,2,3,4,5,6];
    let mut iter = PairIterator::from(&incr);
    assert!(iter.all(|(a,b)|a<b))
}
```
*/
pub struct PairIterator<T, I> where T: Copy , I: Iterator<Item=T> {
    iter: I,
    prev: T
}

impl<T, I> Iterator for  PairIterator<T, I> where T: Copy , I: Iterator<Item=T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.iter.next(){
            Some(curr) =>{
                let tmp = self.prev;
                self.prev = curr;
                Some((tmp, curr))
            }, None => None
        }
    }
}

impl<T, I, J> From<J> for PairIterator<T, I>
    where T: Copy , I: Iterator<Item=T>,
    J: IntoIterator<Item=T, IntoIter=I>
{
    fn from(iter: J) -> Self {
        let mut iter = iter.into_iter();
        let prev = match iter.next() {
            Some(val) => val,
            None => unsafe{uninitialized()} //this is safe - prev won't be used anyway
        };
        Self{
            iter, prev
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_decr(){
        let decr = [1,2,3,4,5,6,7,0];
        let mut iter = PairIterator::from(&decr);
        assert_eq!(iter.position(|(a,b)|a>b), Some(6));
    }

    #[test]
    fn sorted(){
        let incr = [1,2,3,4,5,6];
        let mut iter = PairIterator::from(&incr);
        assert!(iter.all(|(a,b)|a<b))
    }
}