mod pair_iter;
mod partition;

pub use self::pair_iter::PairIterator;
pub (crate) use self::partition::{partition, partition_rand};