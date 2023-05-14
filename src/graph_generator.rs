use std::{fmt::Debug, ops::ControlFlow};

use pipeline::{Pipeline, Request, Time};
use rand::{prelude::Distribution, Rng};

#[derive(Debug)]
pub(crate) struct GraphGenerator<ArrivalDistr, Rand>
where
    ArrivalDistr: Distribution<Time> + Debug,
    Rand: Rng + Debug,
{
    pipeline: Pipeline<ArrivalDistr, Rand>,
    max_x: f32,
    x_step: f32,
    x: f32,
    max_y: f32,
    points: Vec<(f32, f32)>,
}

pub(crate) struct Graph {
    pub points: Vec<(f32, f32)>,
    pub mean: f32,
    pub deviation: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl<ArrivalDistr, Rand> GraphGenerator<ArrivalDistr, Rand>
where
    ArrivalDistr: Distribution<Time> + Debug,
    Rand: Rng + Debug,
{
    pub fn new(pipeline: Pipeline<ArrivalDistr, Rand>, max_x: f32, x_step: f32) -> Self {
        Self {
            pipeline,
            max_x,
            x_step,
            x: 0.0,
            max_y: 0.0,
            points: Vec::with_capacity((max_x / x_step) as usize),
        }
    }

    pub fn generate(mut self) -> Graph {
        loop {
            match self.next() {
                ControlFlow::Break(()) => break,
                ControlFlow::Continue(()) => continue,
            }
        }

        let (mean, deviation) = self.calc_mean_and_deviation(&self.points);

        Graph {
            mean,
            deviation,
            points: self.points,
            max_x: self.max_x,
            max_y: self.max_y,
        }
    }

    fn next(&mut self) -> ControlFlow<(), ()> {
        if self.x > self.max_x {
            return ControlFlow::Break(());
        }

        if self.x == 0.0 {
            self.points.push((0.0, 0.0));
            self.x = self.x_step;
            return ControlFlow::Continue(());
        }

        self.pipeline.reset();
        let requests = self.pipeline.work_during(Time::from(self.x));

        let average_time = self.calc_average_time(&requests);
        if average_time > self.max_y {
            self.max_y = average_time;
        }

        self.points.push((f32::from(self.x), average_time));
        self.x += self.x_step;

        ControlFlow::Continue(())
    }

    fn calc_average_time(&self, requests: &[Request]) -> f32 {
        let requests_num = requests.len() as f32;

        requests
            .into_iter()
            .map(|req| f32::from(req.leaving_time - req.arrival_time))
            .sum::<f32>()
            / requests_num
    }

    fn calc_mean_and_deviation(&self, points: &[(f32, f32)]) -> (f32, f32) {
        let sum = points.iter().map(|(_, y)| y).sum::<f32>();
        let mean = sum / points.len() as f32;

        let dispersion =
            points.iter().map(|(_, y)| (y - mean).powi(2)).sum::<f32>() / points.len() as f32;
        let deviation = dispersion.sqrt();

        (mean, deviation)
    }
}
