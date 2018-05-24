/*!
Searching algorithms.

*/

mod binary_search;
pub mod quick_select;
pub mod interpolation_search;
mod binary_first;

pub use self::binary_search::{binary_search, binary_search_by};
pub use self::binary_first::{binary_first_by};
