/*!
Sorting and related algorithms.
*/

mod is_ordered;
mod first_unordered;
mod counting_sort;
mod longest_substring;
pub mod longest_subsequence;
mod order;
mod shuffle;
mod quick_sort;

pub use self::order::{SortingOrder, Order};
pub use self::counting_sort::{counting_sort_by, counting_sort};
pub use self::first_unordered::{first_unordered, first_unordered_by};
pub use self::is_ordered::{is_ordered, is_ordered_by};
pub use self::longest_substring::{longest_ordered_substring, longest_ordered_substring_by};
pub use self::shuffle::shuffle;
pub use self::quick_sort::{quick_sort_by, quick_sort, quick_sort_rand_by, quick_sort_rand};



