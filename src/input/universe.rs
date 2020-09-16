//! Simulation input conglomerate.

use crate::input::Settings;
use arctk::{
    geom::{Grid, Mesh, Tree},
    ord::Set,
};

/// MCRT simulation resources conglomerate.
pub struct Universe<'a, T: Ord> {
    /// Adaptive tree.
    pub tree: &'a Tree<'a, T>,
    /// Regular grid.
    pub grid: &'a Grid,
    /// Engine settings.
    pub sett: &'a Settings,
    /// Surfaces.
    pub surfs: &'a Set<T, Mesh>,
    // /// Attributes.
    // pub attrs: &'a Set<T, Attributes>,
    // /// Materials.
    // pub mats: &'a Set<T, Material>,
}

impl<'a, T: Ord> Universe<'a, T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        tree: &'a Tree<'a, T>,
        grid: &'a Grid,
        sett: &'a Settings,
        surfs: &'a Set<T, Mesh>,
        // attrs: &'a Set<T, Attributes>,
        // mats: &'a Set<T, Material>,
    ) -> Self {
        Self {
            tree,
            grid,
            sett,
            surfs,
            // attrs,
            // mats,
        }
    }
}
