pub type Vector2f = (f64, f64);

pub trait Norm<T> {
    fn norm(&self) -> T;
}

impl Norm<f64> for Vector2f {
    fn norm(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
}
