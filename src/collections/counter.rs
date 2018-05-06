use std::hash::{Hash, BuildHasher};
use std::iter::{FromIterator, Extend};
use std::default::Default;
use std::ops::{Deref, DerefMut, Add, AddAssign, Sub, SubAssign};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Counter<T, S = RandomState> where T: Hash + Eq, S: BuildHasher {
    //pub type Map = HashMap<T, usize, S>
    counter: HashMap<T, usize, S>
}

impl<T, S> Counter<T, S>  where T: Hash + Eq, S: BuildHasher
{
    /// Create a new, empty `Counter`
    pub fn new() -> Counter<T, RandomState> {
        Counter{
            counter: HashMap::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> Counter<T, RandomState>  {
        Counter{
            counter: HashMap::with_capacity(capacity)
        }
    }

    pub fn with_hasher(hash_builder: S) -> Counter<T, S> {
        Counter{
            counter: HashMap::with_hasher(hash_builder)
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S,  ) -> Counter<T, S>{
        Counter{
            counter: HashMap::with_capacity_and_hasher(capacity, hash_builder)
        }
    }
}

impl<T, S> Default for Counter<T, S> where T: Hash + Eq, S: BuildHasher+Default {

    /// Create a new, empty `Counter`
    fn default() -> Self {
        Counter{
            counter: HashMap::default()
        }
    }
}

impl<T, S> FromIterator<T> for Counter<T, S> where T: Hash + Eq, S: BuildHasher+Default{
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut map = HashMap::default();
        for key in iter{
            *map.entry(key).or_insert(0) +=1;
        }
        Counter{
            counter: map
        }
    }
}

impl<T, S> Extend<T> for Counter<T, S> where T: Hash + Eq, S: BuildHasher {
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        for key in iter{
            *self.counter.entry(key).or_insert(0) +=1;
        }
    }
}


impl<'a, T, S> Extend<&'a T> for Counter<T, S> where T: Hash + Eq+Copy, S: BuildHasher {
    fn extend<I: IntoIterator<Item=&'a T>>(&mut self, iter: I) {
        for key in iter.into_iter().map(|&key| key){
            *self.counter.entry(key).or_insert(0) +=1;
        }
    }
}

impl<T, S> Deref for Counter<T, S> where T: Hash + Eq, S: BuildHasher{
    type Target = HashMap<T, usize, S>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.counter
    }
}

impl<T, S> DerefMut for Counter<T, S> where T: Hash + Eq, S: BuildHasher{
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        & mut self.counter
    }
}
/*
impl<T> AddAssign for Counter<T> where T: Hash + Eq  {
    fn add_assign(&mut self, rhs: Self) {
        let iter = rhs.into_iter();

        for (key, val) in rhs.into_iter(){
            *self.counter.entry(key).or_insert(0) += val;
        }
    }
}
*/