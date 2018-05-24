/*!
Sorting and related algorithms.
*/

mod is_ordered;
mod first_unordered;
mod counting_sort;
pub mod longest_substring;
pub mod longest_subsequence;
mod order;

pub use self::order::{SortingOrder, Order};
pub use self::counting_sort::{counting_sort_by, counting_sort};
pub use self::first_unordered::{first_unordered, first_unordered_by};
pub use self::is_ordered::{is_ordered, is_ordered_by};

