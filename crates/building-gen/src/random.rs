use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

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
