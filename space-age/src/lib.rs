const ONE_EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

pub trait Planet {
    const ONE_YEAR_FACTOR_EARTH: f64;
    fn years_during(d: &Duration) -> f64 {
        (d.0 / ONE_EARTH_YEAR_IN_SECONDS) / Self::ONE_YEAR_FACTOR_EARTH
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const ONE_YEAR_FACTOR_EARTH: f64 = 0.240_846_7;
}
impl Planet for Venus {
    const ONE_YEAR_FACTOR_EARTH: f64 = 0.615_197_26;
}
impl Planet for Earth {
    const ONE_YEAR_FACTOR_EARTH: f64 = 1.0;
}
impl Planet for Mars {
    const ONE_YEAR_FACTOR_EARTH: f64 = 1.880_815_8;
}
impl Planet for Jupiter {
    const ONE_YEAR_FACTOR_EARTH: f64 = 11.862_615;
}
impl Planet for Saturn {
    const ONE_YEAR_FACTOR_EARTH: f64 = 29.447_498;
}
impl Planet for Uranus {
    const ONE_YEAR_FACTOR_EARTH: f64 = 84.016_846;
}
impl Planet for Neptune {
    const ONE_YEAR_FACTOR_EARTH: f64 = 164.791_32;
}
