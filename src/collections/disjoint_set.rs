use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};
use std::iter::{Extend, FromIterator};
use std::default::Default;

#[derive(Debug, Clone, Copy)]
struct Data {
    pub parent: usize,
    pub rank: u32
}

impl Data {
    pub fn new(id: usize) -> Data {
        Data {
            parent: id,
            rank: 0
        }
    }
}

/**
Implementation of disjoint set data structure with path compression and union by rank.

This data structure is also known as union-find or merge-find set.
It tracks a set of elements partitioned into a number of disjoint (non-overlapping) subsets.

**More:** <https://en.wikipedia.org/wiki/Disjoint-set_data_structure>

# Complexity

- Create new subset complexity: O(1)
- Union complexity: O(α(n)) ≈ O(1)
- Search complexity: O(α(n)) ≈ O(1)
- Memory complexity: O(n)

where α() - very slowly growing function. α(n) < 4 for any reasonable n.
Therefore O(α(n)) ≈ O(1).

# Example

```
extern crate algorithm;
use algorithm::collections::DisjointSet;
use std::iter::FromIterator;

fn main(){
   let arr = [1,2,3,4,5,6,7];

   //creates 7 disjoint sets
   let mut ds: DisjointSet<i32> = DisjointSet::from_iter(&arr);

    //you can join existing sets
    ds.union(1, 2);

    //or add elements to existing sets
    ds.union(1,8);

    //you can check if elements are in the same set
    assert!(ds.in_union(&2,&8));
    assert!(!ds.in_union(&3,&4));

    //or if the element has been previously added to the set
    assert!(ds.contains(&7));
    assert!(!ds.contains(&10));
}
```
*/
#[derive(Clone, Debug)]
pub struct DisjointSet<T, S=RandomState>  where T: Eq+Hash , S: BuildHasher{
    ids: HashMap<T, usize, S>,
    data_by_id: Vec<Data>
}

impl<T, S> DisjointSet<T, S> where T:Eq + Hash , S:BuildHasher{

    /// Creates a new, empty `DisjointSet`.
    pub fn new() -> DisjointSet<T, RandomState> where S: Default{
        Default::default()
    }

    /**
    Creates an empty DisjointSet with the specified capacity.

    The DisjointSet will be able to hold at least capacity elements without reallocating.
    If capacity is 0, the DisjointSet will not allocate.
    */
    pub fn with_capacity(capacity: usize) -> DisjointSet<T, RandomState> where S: Default{
        DisjointSet {
            ids: HashMap::with_capacity_and_hasher(capacity, Default::default()),
            data_by_id: Vec::with_capacity(capacity)
        }
    }

    /**
    Creates an empty DisjointSet which will use the given hash builder to hash keys.

    The created set has the default initial capacity.
    */
    pub fn with_hasher(hash_builder: S) -> DisjointSet<T, S> {
        Self {
            ids: HashMap::with_hasher(hash_builder),
            data_by_id: Vec::new()
        }
    }

    /**
    Creates an empty Counter with the specified capacity, using hash_builder to hash the keys.

    The Counter will be able to hold at least capacity elements without reallocating.
    If capacity is 0, the Counter will not allocate.
    */
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> DisjointSet<T, S> {
        Self {
            ids: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            data_by_id: Vec::with_capacity(capacity)
        }
    }

    /**
    Crates a subset with the provided element.

    If the given element already exists, nothing happens.

    **Complexity:**: O(1)
    */
    pub fn make_set(&mut self, val: T) {
        self.make_or_get_set(val);
    }

    /**
    Joins two subsets using one element from both subsets.

    If the provided elements do not exist in the collection when this function is called,
    a new subset with one element gets created prior to joining.

    **Complexity:** O(α(n)) ≈ O(1)
    */
    pub fn union(&mut self, a :T, b: T) {
        let a = self.make_or_get_set(a);
        let b = self.make_or_get_set(b);
        let mut a_root = self.find_with_path_compression(a);
        let mut b_root = self.find_with_path_compression(b);
        if a_root == b_root {
            return;
        }

        if self.data_by_id[a_root].rank < self.data_by_id[b_root].rank {
            let tmp = a_root;
            a_root = b_root;
            b_root = tmp;
        }

        self.data_by_id[b_root].parent = a_root;

        if self.data_by_id[a_root].rank == self.data_by_id[b_root].rank {
            self.data_by_id[a_root].rank += 1;
        }
    }

    /**
    Check if the given element has been added to this collection.

    **Complexity:** O(α(n)) ≈ O(1)
    */
    pub fn contains(&self, val: &T) -> bool {
        self.ids.contains_key(val)
    }

    /**
    Checks if the given two elements are in the same subset.

    **Complexity:** O(α(n)) ≈ O(1)
    */
    pub fn in_union(&mut self, a :&T, b: &T) -> bool{
        let a = match self.ids.get(a) {
            Option::None => return false,
            Option::Some(id) => *id
        };

        let b = match self.ids.get(b) {
            Option::None => return false,
            Option::Some(id) => *id
        };

        self.find_with_path_compression(a) == self.find_with_path_compression(b)
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn clear(&mut self) {
        self.ids.clear();
        self.data_by_id.clear()
    }

    fn make_or_get_set(&mut self, val: T) -> usize{
        let next_id = self.ids.len();
        //insert but do not override existing one
        match self.ids.entry(val) {
            Entry::Vacant(entry) => {
                entry.insert(next_id);
                //make element its own parent
                self.data_by_id.push(Data::new(next_id));
                next_id
            },
            Entry::Occupied(entry) => *entry.get()
        }
    }

    fn find_with_path_compression(&mut self, id: usize) -> usize{
        let mut parent = self.data_by_id[id].parent;
        if parent != id{
            parent = self.find_with_path_compression(parent);
            self.data_by_id[id].parent = parent;
        }
        parent
    }
}

impl<T, S> Default for DisjointSet<T, S>  where T: Eq+Hash , S: BuildHasher + Default {
    fn default() -> Self {

        Self{
            ids: HashMap::default(),
            data_by_id: Vec::default()
        }
    }
}

impl<T, S> FromIterator<T> for DisjointSet<T, S>
    where
        T: Hash + Eq,
        S: BuildHasher + Default,
{
    /**
    Creates DisjointSet from provided iterator.

    Elements become a new subsets with just one element
    (equivalent to calling make_set() multiple times).
    */
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ds: DisjointSet<T, S> = DisjointSet::default();
        for val in iter {
            ds.make_set(val);
        }
        ds
    }
}

impl<'a, T, S> FromIterator<&'a T> for DisjointSet<T, S>
    where
        T: Hash + Eq + Clone,
        S: BuildHasher + Default,
{
    /**
   Creates DisjointSet from provided iterator.

   Elements become a new subsets with just one element
   (equivalent to calling make_set() multiple times).
   */
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        let mut ds = Self::default();
        for val in iter.into_iter().map(|ref val| (*val).clone()) {
            ds.make_set(val)
        }
        ds
    }
}


impl<T, S> Extend<T> for DisjointSet<T, S>
    where
        T: Hash + Eq,
        S: BuildHasher,
{
    /**
   Extends collection using the provided iterator.

   Elements become a new subsets with just one element
   (equivalent to calling make_set() multiple times).
   */
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for val in iter {
            self.make_set(val)
        }
    }
}

impl<'a, T, S> Extend<&'a T> for DisjointSet<T, S>
    where
        T: Hash + Eq + Copy,
        S: BuildHasher,
{
    /**
   Extends collection using the provided iterator.

   Elements become a new subsets with just one element
   (equivalent to calling make_set() multiple times).
   */
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        for val in iter.into_iter().map(|&val| val.clone()) {
            self.make_set(val);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let arr = [1,2,3];
        //let ds = DisjointSet::from_iter(&arr);
        //let ds: DisjointSet<i32, RandomState> = DisjointSet::new();
    }
}