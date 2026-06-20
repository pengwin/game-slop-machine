use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Computes a deterministic unit float [0, 1) from lot geometry and a seed.
/// Uses Wyhash mixing for fast, well-distributed hashing.
pub fn deterministic_lot_unit(pos_x: f32, pos_y: f32, width: f32, depth: f32, seed: u64) -> f32 {
    let mut hash = seed
        ^ (pos_x.to_bits() as u64).wrapping_mul(0x9E37_79B1_85EB_CA87)
        ^ (pos_y.to_bits() as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
        ^ (width.to_bits() as u64).wrapping_mul(0x1656_67B1_9E37_79F9)
        ^ (depth.to_bits() as u64).wrapping_mul(0x85EB_CA77_C2B2_AE63);
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(0xff51_afd7_ed55_8ccd);
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    hash ^= hash >> 33;
    (hash as f64 / u64::MAX as f64) as f32
}

pub struct SeededRng {
    rng: ChaCha8Rng,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    pub fn gen_range(&mut self, min: f32, max: f32) -> f32 {
        self.rng.random_range(min..max)
    }

    pub fn gen_range_usize(&mut self, min: usize, max: usize) -> usize {
        self.rng.random_range(min..max)
    }

    pub fn gen_bool(&mut self, probability: f64) -> bool {
        self.rng.random_bool(probability)
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        use rand::seq::SliceRandom;
        slice.shuffle(&mut self.rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic() {
        let mut rng1 = SeededRng::new(42);
        let mut rng2 = SeededRng::new(42);

        for _ in 0..100 {
            assert_eq!(rng1.gen_range(0.0, 1.0), rng2.gen_range(0.0, 1.0));
        }
    }

    #[test]
    fn test_different_seeds() {
        let mut rng1 = SeededRng::new(42);
        let mut rng2 = SeededRng::new(43);

        let a: Vec<f32> = (0..10).map(|_| rng1.gen_range(0.0, 1.0)).collect();
        let b: Vec<f32> = (0..10).map(|_| rng2.gen_range(0.0, 1.0)).collect();

        assert_ne!(a, b);
    }
}
