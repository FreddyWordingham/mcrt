//! Running engines.

use crate::{
    input::Universe,
    output::{Data, Sample},
    parts::{Key, Photon},
};
use rand::rngs::ThreadRng;

pub type Engine = dyn Fn(&Universe<Key>, &mut ThreadRng, &mut Data, Photon) -> Sample;

pub mod default;
pub mod raman;

pub use self::default::*;
pub use self::raman::*;
