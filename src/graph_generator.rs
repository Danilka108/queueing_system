use std::fmt::Debug;

use pipeline::{Pipeline, Time};
use rand::{prelude::Distribution, Rng};

#[derive(Debug)]
pub(crate) struct GraphGenerator<ArrivalDistr, Rand>
where
    ArrivalDistr: Distribution<Time> + Debug,
    Rand: Rng + Debug,
{
    pipeline: Pipeline<ArrivalDistr, Rand>,
}

impl<ArrivalDistr, Rand> GraphGenerator<ArrivalDistr, Rand>
where
    ArrivalDistr: Distribution<Time> + Debug,
    Rand: Rng + Debug,
{
    pub fn new(pipeline: Pipeline<ArrivalDistr, Rand>) -> Self {
        Self { pipeline }
    }

    pub fn generate(&mut self, max_x: f32, x_step: f32) -> (f32, Vec<(f32, f32)>) {
        let mut x_i = 0.0;
        let mut points = Vec::with_capacity((max_x / x_step) as usize);
        let mut max_y = 0.0;

        loop {
            if x_i > max_x {
                break;
            }

            if x_i == 0.0 {
                points.push((0.0, 0.0));
                x_i = x_step;
                continue;
            }

            self.pipeline.reset();

            let requests = self.pipeline.work_during(Time::from(x_i));
            let requests_num = requests.len() as f32;

            let average_time = requests
                .into_iter()
                .map(|req| f32::from(req.leaving_time - req.arrival_time))
                .sum::<f32>()
                / requests_num;

            if average_time > max_y {
                max_y = average_time;
            }

            points.push((f32::from(x_i), average_time));
            x_i += x_step;
        }

        (max_y, points)
    }
}
