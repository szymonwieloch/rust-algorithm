/*!
Searching algorithms.

*/

mod binary_search;
mod quick_select;
mod interpolation_search;
mod binary_first;

pub use self::binary_search::{binary_search, binary_search_by};
pub use self::binary_first::{binary_first_by};
pub use self::interpolation_search::{interpolation_search, interpolation_search_by};
pub use self::quick_select::{quick_select, quick_select_rand, quick_select_by, quick_select_rand_by};
