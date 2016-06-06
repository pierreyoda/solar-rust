pub type Vector2f = (f64, f64);

pub trait Vector<T> {
    fn plus(&self, other: &Self) -> Self;
    fn minus(&self, other: &Self) -> Self;

    fn norm(&self) -> f64;
}

impl Vector<f64> for Vector2f {
    fn plus(&self, other: &Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }

    fn minus(&self, other: &Self) -> Self {
        (self.0 - other.0, self.1 - other.1)
    }

    fn norm(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
}
