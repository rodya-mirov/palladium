#![allow(deprecated)]
// use deprecated RNG because the rand team screwed up, the non-deprecated thing is broken
// that's what you get with pre-1.0 software i guess

pub use rand::Rng;
use rand::{Isaac64Rng, SeedableRng};

pub type PalladRng = Isaac64Rng;

pub fn make_rng(seed: u64) -> PalladRng {
    Isaac64Rng::seed_from_u64(seed)
}
