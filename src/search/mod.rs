/*!
Searching algorithms.

*/

mod binary_search;
pub mod quick_select;
pub mod interpolation_search;

pub use self::binary_search::{binary_search, binary_search_by};
