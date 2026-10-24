//! Unit-typed values and conversions for cosmological and local scales.
//!
//! The simulation separates **cosmic scale** (megaparsecs) from **planetary
//! scale** (meters/kilometers/AU) to avoid f64 precision loss when mixing
//! vastly different magnitudes in a single global frame.
//!
//! These types are markers and converters — they do not yet carry full vector
//! math. That is added incrementally as coordinate-frame systems arrive.

/// A length value in meters (the SI base unit).
///
/// Used for planetary-scale distances where sub-meter precision is needed.
pub struct Length {
    pub meters: f64,
}

impl Length {
    pub fn new(meters: f64) -> Self {
        Self { meters }
    }

    /// Convert meters to kilometers.
    pub fn kilometers(&self) -> f64 {
        self.meters / 1_000.0
    }

    /// Convert meters to astronomical units (AU).
    pub fn au(&self) -> f64 {
        self.meters / 149_597_870_700.0 // IAU 2012 definition
    }
}

/// A distance in parsecs (pc), used for galactic and stellar scales.
pub struct Parsec {
    pub parsecs: f64,
}

impl Parsec {
    pub fn new(parsecs: f64) -> Self {
        Self { parsecs }
    }

    /// Convert parsecs to meters: 1 pc = 648000/π * AU ≈ 3.0857×10¹⁶ m
    pub fn meters(&self) -> f64 {
        // IAU nominal parsec ≈ 3.0856775814913673 × 10^16 m
        self.parsecs * 3.0856775814913673e16
    }

    /// Convert parsecs to light-years.
    pub fn light_years(&self) -> f64 {
        // 1 pc ≈ 3.2407804897203643 ly (Julian year)
        self.parsecs * 3.2615637773475861
    }

    /// Convert parsecs to AU.
    pub fn au(&self) -> f64 {
        self.meters() / 149_597_870_700.0
    }
}

/// A distance in megaparsecs (Mpc), used for cosmic / intergalactic scales.
pub struct Megaparsec {
    pub megaparsecs: f64,
}

impl Megaparsec {
    pub fn new(megaparsecs: f64) -> Self {
        Self { megaparsecs }
    }

    /// Convert Mpc to parsecs.
    pub fn parsecs(&self) -> f64 {
        self.megaparsecs * 1.0e6
    }

    /// Convert Mpc to meters.
    pub fn meters(&self) -> f64 {
        Parsec::new(self.parsecs()).meters()
    }

    /// Convert Mpc to light-years.
    pub fn light_years(&self) -> f64 {
        Parsec::new(self.parsecs()).light_years()
    }

    /// Convert Mpc to AU.
    pub fn au(&self) -> f64 {
        Parsec::new(self.parsecs()).au()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsec_to_meters_and_back() {
        let pc = Parsec::new(1.0);
        let m = pc.meters();
        // 1 parsec ≈ 3.0857 × 10^16 m
        assert!((m - 3.0857_0e16).abs() / 3.0857_0e16 < 1e-5);

        // Round-trip through meters is not exact due to f64 precision,
        // but the relative error should be negligible.
        let back = Parsec::new(m / 3.0856775814913673e16);
        assert!((back.parsecs - 1.0).abs() < 1e-10);
    }

    #[test]
    fn megaparsec_to_parsec() {
        let mpc = Megaparsec::new(1.0);
        assert_eq!(mpc.parsecs(), 1.0e6);
    }

    #[test]
    fn megaparsec_to_au() {
        // 1 Mpc ≈ 2.06265 × 10^11 AU (approximate, for sanity-check order of magnitude)
        let mpc = Megaparsec::new(1.0);
        let au = mpc.au();
        assert!(au > 2.0e11 && au < 2.1e11);
    }

    #[test]
    fn length_au_conversion() {
        let earth_sun = Length::new(149_597_870_700.0);
        assert!((earth_sun.au() - 1.0).abs() < 1e-15);
    }

    #[test]
    fn length_kilometers() {
        let km = Length::new(5_000.0);
        assert!((km.kilometers() - 5.0).abs() < 1e-15);
    }

    #[test]
    fn parsec_to_light_years() {
        let pc = Parsec::new(1.0);
        // 1 pc ≈ 3.26 ly
        assert!((pc.light_years() - 3.26156).abs() < 0.01);
    }

    #[test]
    fn zero_values_are_valid() {
        let zero_pc = Parsec::new(0.0);
        assert_eq!(zero_pc.meters(), 0.0);
        assert_eq!(zero_pc.light_years(), 0.0);

        let zero_mpc = Megaparsec::new(0.0);
        assert_eq!(zero_mpc.meters(), 0.0);

        let zero_len = Length::new(0.0);
        assert_eq!(zero_len.au(), 0.0);
    }
}
