use rand::Rng;
use rand::distributions::{Normal, Range, IndependentSample};

pub type SamplerFunction<T, R> = Box<Fn(&mut R) -> T>;

/// Randomly generate instances of a certain type.
pub trait TypeGenerator<R: Rng> {
    type Generated;

    fn default() -> Self;

    fn generate(&mut self, rng: &mut R) -> Result<Self::Generated, String>;

    fn generate_many(&mut self, rng: &mut R, n: usize) -> Result<Vec<Self::Generated>, String> {
        let mut generated = Vec::with_capacity(n);
        for _ in 0..n {
            generated.push(try!(self.generate(rng)));
        }
        Ok(generated)
    }
}

/// Specifies a random distribution.
pub enum Distribution {
    Constant(f64),
    Normal {
        mean: f64,
        std_dev: f64,
    },
    Range {
        low: f64,
        high: f64,
    },
}

impl Distribution {
    /// Create from the specifications the rand-crate based distribution samplers.
    pub fn to_sampler<R: Rng>(&self) -> SamplerFunction<f64, R> {
        match self {
            &Distribution::Constant(value) => Box::new(move |_| value),
            &Distribution::Normal { mean, std_dev } => {
                let normal = Normal::new(mean, std_dev);
                Box::new(move |rng: &mut R| normal.ind_sample(rng))
            }
            &Distribution::Range { low, high } => {
                let range = Range::new(low, high);
                Box::new(move |rng: &mut R| range.ind_sample(rng))
            }
        }
    }
}

impl<R: Rng> Into<SamplerFunction<f64, R>> for Distribution {
    fn into(self) -> SamplerFunction<f64, R> {
        self.to_sampler()
    }
}

/// Generates random RGB colors, with each component having a value between 0.0 and 1.0.
pub struct ColorGenerator<R: Rng> {
    pub r: SamplerFunction<f64, R>,
    pub g: SamplerFunction<f64, R>,
    pub b: SamplerFunction<f64, R>,
}

impl<R: Rng> TypeGenerator<R> for ColorGenerator<R> {
    type Generated = [f32; 4];

    fn default() -> Self {
        ColorGenerator {
            r: Distribution::Range {
                   low: 0.0,
                   high: 1.0,
               }
               .into(),
            g: Distribution::Range {
                   low: 0.0,
                   high: 1.0,
               }
               .into(),
            b: Distribution::Range {
                   low: 0.0,
                   high: 1.0,
               }
               .into(),
        }
    }

    fn generate(&mut self, rng: &mut R) -> Result<Self::Generated, String> {
        Ok([(self.r)(rng) as f32, (self.g)(rng) as f32, (self.b)(rng) as f32, 1.0])
    }
}

#[macro_export]
macro_rules! generator_sample {
    ($self_: ident, $sampler_opt_fn: ident, $rng: ident) => (
        ($self_.$sampler_opt_fn.as_ref().unwrap())($rng)
    )
}
