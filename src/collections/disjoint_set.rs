use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};
use std::iter::{Extend, FromIterator};
use std::default::Default;

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


pub struct DisjointSet<T, S=RandomState>  where T: Eq+Hash , S: BuildHasher{
    ids: HashMap<T, usize, S>,
    data_by_id: Vec<Data>
}

impl<T, S> DisjointSet<T, S> where T:Eq + Hash , S:BuildHasher{

    pub fn new() -> DisjointSet<T, RandomState> {
        DisjointSet {
            ids: HashMap::new(),
            data_by_id: Vec::new()
        }
    }

    /**
Creates an empty DisjointSet with the specified capacity.

The DisjointSet will be able to hold at least capacity elements without reallocating.
If capacity is 0, the DisjointSet will not allocate.
*/

    pub fn with_capacity(capacity: usize) -> DisjointSet<T, RandomState> {
        DisjointSet {
            ids: HashMap::with_capacity(capacity),
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

    pub fn make_set(&mut self, val: T) {
        self.make_or_get_set(val);
    }

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
    ///Creates DisjointSet from provided iterator.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ds = Self::default();
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
    ///Creates DisjointSet from provided iterator.
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
    ///Extends Counter with provided interator.
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
    ///Extends Counter with provided interator.
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        for val in iter.into_iter().map(|&val| val.clone()) {
            self.make_set(val);
        }
    }
}
