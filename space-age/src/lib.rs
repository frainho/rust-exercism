const ONE_EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;

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
        Earth::years_during(d) / Self::ONE_YEAR_FACTOR_EARTH
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
    const ONE_YEAR_FACTOR_EARTH: f64 = 0.2408467;
}
impl Planet for Venus {
    const ONE_YEAR_FACTOR_EARTH: f64 = 0.61519726;
}
impl Planet for Earth {
    const ONE_YEAR_FACTOR_EARTH: f64 = 1.0;
    fn years_during(d: &Duration) -> f64 {
        d.0 / ONE_EARTH_YEAR_IN_SECONDS
    }
}
impl Planet for Mars {
    const ONE_YEAR_FACTOR_EARTH: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const ONE_YEAR_FACTOR_EARTH: f64 = 11.862615;
}
impl Planet for Saturn {
    const ONE_YEAR_FACTOR_EARTH: f64 = 29.447498;
}
impl Planet for Uranus {
    const ONE_YEAR_FACTOR_EARTH: f64 = 84.016846;
}
impl Planet for Neptune {
    const ONE_YEAR_FACTOR_EARTH: f64 = 164.79132;
}
