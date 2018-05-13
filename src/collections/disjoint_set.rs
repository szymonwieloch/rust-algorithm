use std::hash::Hash;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct Node {
    pub parent: usize,
    pub rank: u32
}

impl Node {
    pub fn new(id: usize) -> Node {
        Node{
            parent: id,
            rank: 0
        }
    }
}


pub struct DisjointSet<T>  where T: Eq+Hash {
    ids: HashMap<T, usize>,
    parent: Vec<Node>
}

impl<T> DisjointSet<T> where T:Eq + Hash {
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

        if self.parent[a_root].rank < self.parent[b_root].rank {
            let tmp = a_root;
            a_root = b_root;
            b_root = tmp;
        }

        self.parent[b_root].parent = a_root;

        if self.parent[a_root].rank == self.parent[b_root].rank {
            self.parent[a_root].rank += 1;
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
            Entry::Vacant(mut entry) => {
                entry.insert(next_id);
                //make element its own parent
                self.parent.push(Node::new(next_id));
                next_id
            },
            Entry::Occupied(entry) => *entry.get()
        }
    }

    fn find_with_path_compression(&mut self, id: usize) -> usize{
        let mut parent = self.parent[id].parent;
        if parent != id{
            parent = self.find_with_path_compression(parent);
            self.parent[id].parent = parent;
        }
        parent
    }


}