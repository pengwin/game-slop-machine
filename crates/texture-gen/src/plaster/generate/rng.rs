use super::math::u32_to_f32;

#[derive(Clone)]
pub(super) struct SmallRng {
    state: u32,
}

impl SmallRng {
    pub(super) const fn new(seed: u32) -> Self {
        let state = if seed == 0 { 1 } else { seed };
        Self { state }
    }

    const fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    pub(super) fn f32(&mut self) -> f32 {
        u32_to_f32(self.next_u32()) / u32_to_f32(u32::MAX)
    }

    pub(super) fn range(&mut self, min: f32, max: f32) -> f32 {
        (max - min).mul_add(self.f32(), min)
    }

    pub(super) fn u32_range(&mut self, min: u32, max: u32) -> u32 {
        min + self.next_u32() % (max - min).max(1)
    }
}
