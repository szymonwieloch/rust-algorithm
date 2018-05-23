/*!
A collection of different algorithms.

This crate aims to provide a set of common algorithms and data structures
used commonly during algorithmic competitions and in complex applications.
All algorithms and data structures are implemented as templates to allow you
to use them with your own custom types.

The following algorithms are already implemented:

# Mathematics

- Median.
- Prefix sums.
- Maximum consecutive sums.

# Sorting

- Finding first unsorted element.
- Checking if the collection is sorted.
- Counting sort.
- Longest sorted subsequence.
- Longest sorted substring.

# Searching

- Binary search.
- Interpolation search.
- Quick select.

# Collections

- Disjoint set (also known as Union Find).
- Counter.

*/

extern crate fnv;
extern crate rand;
pub mod sort;
pub mod search;
pub mod collections;
pub mod math;
pub mod utils;
