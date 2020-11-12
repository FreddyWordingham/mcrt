//! Optical material.

use crate::parts::Emitter;
use arctk::{
    err::Error,
    file::Build,
    geom::{MeshBuilder, Ray},
    math::{Dir3, Pos3},
};
use arctk_attr::input;
use std::path::Path;

/// Ray emission structure.
#[input]
pub enum EmitterBuilder {
    /// Single beam.
    Beam(Pos3, Dir3),
    /// Surface mesh.
    Surface(MeshBuilder),
}

impl Build for EmitterBuilder {
    type Inst = Emitter;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Beam(pos, dir) => Self::Inst::Beam(Ray::new(pos, dir)),
            Self::Surface(mesh) => Self::Inst::Surface(mesh.build(in_dir)?),
        })
    }
}
