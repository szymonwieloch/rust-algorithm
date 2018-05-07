use super::Counter;
use fnv::FnvBuildHasher;

pub type FastCounter<T> = Counter<T, FnvBuildHasher>;