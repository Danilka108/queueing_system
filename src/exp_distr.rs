use pipeline::Time;
use rand::{distributions::Uniform, prelude::Distribution};

#[derive(Debug)]
pub struct ExpDistr {
    mean: f32,
}

impl ExpDistr {
    pub fn new(mean: f32) -> Self {
        Self { mean }
    }
}

impl Distribution<Time> for ExpDistr {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Time {
        let rand_value = rng.sample(Uniform::new(0f32, 1f32));
        let converted_value = -(1f32 - rand_value).ln() * self.mean;
        Time::from(converted_value)
    }
}
