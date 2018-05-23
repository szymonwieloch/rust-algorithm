/*!
Sorting and related algorithms.
*/

pub mod is_sorted;
mod first_unordered;
mod counting_sort;
pub mod longest_substring;
pub mod longest_subsequence;
mod order;

pub use self::order::{SortingOrder, Order};
pub use self::counting_sort::{counting_sort_by, counting_sort};
pub use self::first_unordered::{first_unordered, first_unordered_by};

