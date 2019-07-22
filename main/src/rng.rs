pub use rand::Rng;
use rand::SeedableRng;
use rand_isaac::Isaac64Rng;

pub type PalladRng = Isaac64Rng;

pub fn make_rng(seed: u64) -> PalladRng {
    Isaac64Rng::seed_from_u64(seed)
}
