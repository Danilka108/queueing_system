use std::{fmt::Debug, ops::ControlFlow};

use pipeline::{Pipeline, Request, Statistics, Time};
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
    increase_working_time: bool,
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
    pub fn new(
        pipeline: Pipeline<ArrivalDistr, Rand>,
        increase_working_time: bool,
        max_x: f32,
        x_step: f32,
    ) -> Self {
        Self {
            increase_working_time,
            pipeline,
            max_x,
            x_step,
            x: 0.0,
            max_y: 0.0,
            points: Vec::with_capacity((max_x / x_step) as usize),
        }
    }

    pub fn generate(&mut self) -> Graph {
        self.pipeline.reset();
        self.x = 0.0;
        self.max_y = 0.0;

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
            points: self.points.drain(..).collect(),
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
        let requests = self.pipeline.work_during(if self.increase_working_time {
            Time::from(self.x)
        } else {
            Time::from(self.max_x)
        });

        let average_time = self.calc_average_time(&requests);
        if average_time > self.max_y {
            self.max_y = average_time;
        }

        self.points.push((f32::from(self.x), average_time));
        self.x += self.x_step;

        ControlFlow::Continue(())
    }

    fn calc_average_time(&self, requests: &[Request]) -> f32 {
        if requests.len() == 0 {
            return 0.0;
        }

        requests
            .into_iter()
            .map(|req| f32::from(req.leaving_time - req.arrival_time))
            .sum::<f32>()
            / requests.len() as f32
    }

    fn calc_mean_and_deviation(&self, points: &[(f32, f32)]) -> (f32, f32) {
        let sum = points.iter().map(|(_, y)| *y).sum::<f32>();
        let mean = sum / points.len() as f32;

        let dispersion =
            points.iter().map(|(_, y)| (y - mean).powi(2)).sum::<f32>() / points.len() as f32;
        let deviation = dispersion.sqrt();

        (mean, deviation)
    }
}

pub(crate) fn calc_average_stats<ArrivalDistr, Rand>(
    graph_generator: &mut GraphGenerator<ArrivalDistr, Rand>,
    iters_count: usize,
) -> Statistics
where
    ArrivalDistr: Distribution<Time> + Debug,
    Rand: Rng + Debug,
{
    let mut statistics = Statistics::default();

    for _ in 0..iters_count {
        let Graph { .. } = graph_generator.generate();
        statistics += graph_generator.pipeline.get_statistics();
    }

    statistics /= iters_count as f32;

    statistics
}

pub(crate) fn achive_calc_accuracy<ArrivalDistr, Rand>(
    graph_generator: &mut GraphGenerator<ArrivalDistr, Rand>,
    mut n: f32,
    precision: f32,
    quantile: f32,
) -> f32
where
    ArrivalDistr: Distribution<Time> + Debug,
    Rand: Rng + Debug,
{
    loop {
        let Graph { deviation, .. } = graph_generator.generate();
        let n_asterisk = ((deviation.powi(2) / precision) * quantile).powi(2);

        if n_asterisk > n {
            n = n_asterisk;
            continue;
        }

        break n_asterisk;
    }
}
