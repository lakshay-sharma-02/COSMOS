//! Deterministic seeded random number generation for the simulation.
//!
//! Uses `rand::SmallRng` (ChaCha-based) for speed and small state. Every call
//! to [`SimRng::new`] with the same seed produces an identical stream of
//! random numbers, making simulation ticks reproducible.

use rand::{rngs::SmallRng, SeedableRng};

/// Seeded RNG wrapper for the simulation.
///
/// Constructing with [`SimRng::new`] from a fixed seed yields a deterministic
/// sequence of random values. This is the single source of randomness in
/// `cosmos_sim`.
pub struct SimRng {
    inner: SmallRng,
}

impl SimRng {
    /// Create a new simulation RNG from a 32-byte seed.
    pub fn new(seed: [u8; 32]) -> Self {
        Self {
            inner: SmallRng::from_seed(seed),
        }
    }

    /// Create a new simulation RNG with a fixed default seed.
    /// Used when the caller does not need a custom seed.
    pub fn default_seed() -> Self {
        Self::new(DEFAULT_SEED)
    }

    /// Generate the next u64 value.
    pub fn next_u64(&mut self) -> u64 {
        use rand::RngCore;
        self.inner.next_u64()
    }

    /// Fill a buffer with random bytes.
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        use rand::RngCore;
        self.inner.fill_bytes(dest);
    }

    /// Reset the RNG to a known seed.
    pub fn reseed(&mut self, seed: [u8; 32]) {
        self.inner = SmallRng::from_seed(seed);
    }
}

/// Default 32-byte seed for reproducibility when no seed is specified.
pub const DEFAULT_SEED: [u8; 32] = [
    0x42, 0x2a, 0x1f, 0x8c, 0x73, 0xe6, 0x90, 0xd1, 0xa5, 0xbf, 0x3e, 0x88, 0x12, 0xcd,
    0x54, 0x9a, 0x7b, 0x31, 0xef, 0x80, 0x29, 0x67, 0xbc, 0xd4, 0xa3, 0xf5, 0x18, 0x7e,
    0xc2, 0x09, 0x6b, 0x4f,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_sequence() {
        let seed = [42u8; 32];
        let mut rng_a = SimRng::new(seed);
        let mut rng_b = SimRng::new(seed);

        for _ in 0..100 {
            assert_eq!(rng_a.next_u64(), rng_b.next_u64());
        }
    }

    #[test]
    fn different_seed_different_sequence() {
        let mut rng_a = SimRng::new([1u8; 32]);
        let mut rng_b = SimRng::new([2u8; 32]);

        let a = rng_a.next_u64();
        let b = rng_b.next_u64();
        // Extremely unlikely to collide.
        assert_ne!(a, b);
    }

    #[test]
    fn reseed_produces_same_sequence() {
        let seed = [7u8; 32];
        let mut rng = SimRng::new(seed);
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();

        // Reset and verify we get the same values again.
        rng.reseed(seed);
        assert_eq!(rng.next_u64(), v1);
        assert_eq!(rng.next_u64(), v2);
    }

    #[test]
    fn fill_bytes_deterministic() {
        let seed = [0xAB; 32];
        let mut a = SimRng::new(seed);
        let mut b = SimRng::new(seed);

        let mut buf_a = [0u8; 32];
        let mut buf_b = [0u8; 32];
        a.fill_bytes(&mut buf_a);
        b.fill_bytes(&mut buf_b);

        assert_eq!(buf_a, buf_b);
    }
}
