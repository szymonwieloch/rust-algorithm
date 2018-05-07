use std::hash::{Hash, BuildHasher};
use std::iter::{FromIterator, Extend};
use std::default::Default;
use std::ops::{Deref, DerefMut, Add, AddAssign, Sub, SubAssign};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::collections::hash_map::Entry;

type IntoIter<T> = ::std::collections::hash_map::IntoIter<T, usize>;
type Iter<'a, T> = ::std::collections::hash_map::Iter<'a, T, usize>;
type IterMut<'a, T> = ::std::collections::hash_map::IterMut<'a, T, usize>;

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

    pub fn from_hashmap(rhs: HashMap<T, usize, S>) -> Self {
        Self{
            counter: rhs
        }
    }

    pub fn into_most_common(self) -> Vec<(T, usize)>{
        let mut res: Vec<(T, usize)> = Vec::from_iter(self.counter.into_iter());
        res.sort_unstable_by_key(|&(ref _key, val)| val);
        res
    }

    pub fn most_common(&self) -> Vec<(T, usize)> where T: Clone{
        let mut res: Vec<(T, usize)> = self.counter.iter().map(|(key, &val)|((*key).clone(), val)).collect();
        res.sort_unstable_by_key(|&(ref _key, val)| val);
        res
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

impl <T, S> IntoIterator for Counter<T, S>  where T: Hash + Eq, S: BuildHasher {
    type Item = (T, usize);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.counter.into_iter()
    }
}

impl <'a, T, S> IntoIterator for &'a Counter<T, S>  where T: Hash + Eq, S: BuildHasher {
    type Item = (&'a T,&'a usize);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.counter.iter()
    }
}

impl <'a, T, S> IntoIterator for &'a mut Counter<T, S>  where T: Hash + Eq, S: BuildHasher {
    type Item = (&'a T,&'a mut usize);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.counter.iter_mut()
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

impl<T, S1, S2> AddAssign<Counter<T, S1>> for Counter<T, S2> where T: Hash + Eq, S1: BuildHasher, S2: BuildHasher{
    fn add_assign(&mut self, rhs: Counter<T, S1>) {
        for (key, val) in rhs.into_iter(){
            * self.counter.entry(key).or_insert(0) += val;
        }
    }
}

impl<'a, T, S1, S2> AddAssign<&'a Counter<T, S1>> for Counter<T, S2> where T: Hash + Eq+Clone, S1: BuildHasher, S2: BuildHasher{
    fn add_assign(&mut self, rhs: &'a Counter<T, S1>) {
        for (ref key, &val) in rhs.iter(){
            *self.counter.entry((*key).clone()).or_insert(0) += val;
        }
    }
}

impl<T, S1, S2> Add<Counter<T, S1>> for  Counter<T, S2> where T: Hash + Eq, S1: BuildHasher,  S2: BuildHasher {
    type Output = Counter<T, S2>;
    fn add(mut self, rhs: Counter<T, S1>) -> <Self as Add<Self>>::Output {
        self += rhs;
        self
    }
}

impl<'a, T, S1, S2> Add<&'a Counter<T, S1>> for  Counter<T, S2> where T: Hash + Eq+Clone, S1: BuildHasher, S2: BuildHasher {
    type Output = Counter<T, S2>;
    fn add(mut self, rhs: &'a Counter<T, S1>) -> <Self as Add<Self>>::Output {
        for (ref key, val) in rhs.iter(){
            *self.entry((*key).clone()).or_insert(0) += *val;
        }
        self
    }
}

impl<T, S1, S2> SubAssign<Counter<T, S1>> for Counter<T, S2> where T: Hash + Eq, S1: BuildHasher, S2: BuildHasher{
    fn sub_assign(&mut self, rhs: Counter<T, S1>) {
        for (key, val) in rhs.into_iter(){
            match self.counter.entry(key) {
                Entry::Occupied(mut entry) =>{
                    if entry.get() <= &val {
                        entry.remove();
                    } else {
                        let new_val = entry.get()-val;
                        entry.insert(new_val);
                    }
                },
                Entry::Vacant(..) => {
                    //do nothing - discard
                }
            }
        }
    }
}

impl<'a, T, S1, S2> SubAssign<&'a Counter<T, S1>> for Counter<T, S2> where T: Hash + Eq+Clone, S1: BuildHasher, S2: BuildHasher{
    fn sub_assign(&mut self, rhs: & 'a Counter<T, S1>) {
        for (key, val) in rhs.into_iter(){
            match self.counter.entry(key.clone()) {
                Entry::Occupied(mut entry) =>{
                    if entry.get() <= &val {
                        entry.remove();
                    } else {
                        let new_val = entry.get()-val;
                        entry.insert(new_val);
                    }
                },
                Entry::Vacant(..) => {
                    //do nothing - discard
                }
            }
        }
    }
}

impl<T, S1, S2> Sub<Counter<T, S1>> for  Counter<T, S2> where T: Hash + Eq, S1: BuildHasher,  S2: BuildHasher {
    type Output = Counter<T, S2>;
    fn sub(mut self, rhs: Counter<T, S1>) -> <Self as Sub<Self>>::Output {
        self -= rhs;
        self
    }
}

impl<'a, T, S1, S2> Sub<&'a Counter<T, S1>> for  Counter<T, S2> where T: Hash + Eq+Clone, S1: BuildHasher, S2: BuildHasher {
    type Output = Counter<T, S2>;
    fn sub(mut self, rhs: &'a Counter<T, S1>) -> <Self as Sub<Self>>::Output {
        for (ref key, val) in rhs.iter(){
            match self.counter.entry((*key).clone()) {
                Entry::Occupied(mut entry) =>{
                    if entry.get() <= &val {
                        entry.remove();
                    } else {
                        let new_val = entry.get()-val;
                        entry.insert(new_val);
                    }
                },
                Entry::Vacant(..) => {
                    //do nothing - discard
                }
            }
        }
        self
    }
}

impl <T, S1, S2> From<HashMap<T, usize, S1>> for Counter<T, S2> where T: Hash + Eq, S1: BuildHasher, S2: BuildHasher+Default {
    fn from(rhs: HashMap<T, usize, S1>) -> Self {
        Counter{
            counter: HashMap::from_iter(rhs.into_iter())
        }
    }
}

impl <'a, T, S1, S2> From<&'a HashMap<T, usize, S1>> for Counter<T, S2> where T: Hash + Eq+Clone, S1: BuildHasher, S2: BuildHasher+Default {
    fn from(rhs: &'a HashMap<T, usize, S1>) -> Self {
        Counter{
            counter: HashMap::from_iter(rhs.iter().map(|(ref key, &val)| ((*key).clone(), val)))
        }
    }
}