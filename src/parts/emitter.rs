//! Optical material.

use arctk::geom::{Emit, Mesh, Ray};
use rand::Rng;

/// Ray emission structure.
pub enum Emitter {
    /// Single beam.
    Beam(Ray),
    /// Surface mesh.
    Surface(Mesh),
}

impl Emitter {
    /// Emit a new ray.
    #[inline]
    #[must_use]
    pub fn emit<R: Rng>(&self, rng: &mut R) -> Ray {
        match *self {
            Self::Beam(ref ray) => ray.clone(),
            Self::Surface(ref mesh) => mesh.cast(rng),
        }
    }
}
