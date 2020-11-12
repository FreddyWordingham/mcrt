//! Light surface structure.

use crate::parts::{EmitterBuilder, Light};
use arctk::{err::Error, file::Build, file::Redirect, math::ProbabilityBuilder};
use arctk_attr::input;
use std::path::Path;

/// Loadable light structure.
#[input]
pub struct LightBuilder {
    /// Power [J/s].
    power: f64,
    /// Object path link.
    emit: EmitterBuilder,
    /// Emission form.
    spec: Redirect<ProbabilityBuilder>,
}

impl Build for LightBuilder {
    type Inst = Light;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let emit = self.emit.build(in_dir)?;
        let spec = self.spec.build(in_dir)?.build(in_dir)?;

        Ok(Self::Inst::new(self.power, emit, spec))
    }
}
