//! MCRT settings.

use crate::parts::Key;
use arctk::tools::Range;
use arctk_attr::input;

/// MCRT settings structure.
#[input]
pub struct Settings {
    /// Number of photons to simulate in each thread block.
    pub block_size: u64,
    /// Number of photons to simulate.
    pub num_phot: u64,
    /// Bump distance [m].
    pub bump_dist: f64,
    /// Loop limit.
    pub loop_limit: u64,
    /// Weight to perform roulette at.
    pub roulette_weight: f64,
    /// Number of roulette barrels.
    pub roulette_barrels: u64,
    /// Initial material.
    pub init_mat: Key,
    /// Wavelength limits.
    pub range: Range,
    /// Histogram resolution.
    pub hist_bins: u64,
}
