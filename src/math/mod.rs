/*!
    Mathematical algorithms.

*/

mod max_consecutive_sum;
mod median;
mod prefix_sum;
pub use self::prefix_sum::PrefixSum;
pub use self::max_consecutive_sum::{max_consecutive_sum, max_consecutive_sum_idx};
pub use self::median::{median_rand, median_avg_rand, median, median_avg};
