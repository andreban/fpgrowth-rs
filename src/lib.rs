mod association_rules;
pub(crate) mod fpgrowth;
#[cfg(feature = "parallel")]
mod parallel_fpgrowth;

pub use association_rules::generate_association_rules;
pub use fpgrowth::fp_growth;

#[cfg(feature = "parallel")]
pub use parallel_fpgrowth::parallel_fp_growth;
