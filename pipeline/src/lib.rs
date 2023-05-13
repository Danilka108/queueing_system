pub mod node;
mod request;
mod requests_accum;
mod time;

use rand::{prelude::Distribution, Rng};
use std::fmt::Debug;

pub use crate::request::Request;
pub use crate::time::Time;

#[derive(Debug)]
pub struct PipelineParams<ArrivalDistr, Rand>
where
    Rand: Rng,
{
    pub arrival_distr: ArrivalDistr,
    pub rand_gen: Rand,
}

#[derive(Debug)]
pub struct Pipeline<ArrivalDistr, Rand>
where
    ArrivalDistr: Distribution<Time>,
    Rand: Rng,
{
    arrival_distr: ArrivalDistr,
    requests_accum: requests_accum::RequestsAccum,
    delayed_requests_count: usize,
    start_node: Box<dyn node::PipelineNode>,
    rng: Rand,
}

impl<ArrivalDistr, Rand> PipelineParams<ArrivalDistr, Rand>
where
    ArrivalDistr: Distribution<Time>,
    Rand: Rng + Clone,
{
    pub fn build(
        self,
        nodes: Vec<Box<dyn node::IntoPipelineNode<Rand>>>,
    ) -> Pipeline<ArrivalDistr, Rand> {
        let (requests_accum, mut start_node): (_, Box<dyn node::PipelineNode>) =
            requests_accum::requests_accum();

        for node in nodes.into_iter().rev() {
            start_node = node.into_node(self.rand_gen.clone(), start_node);
        }

        Pipeline {
            start_node,
            arrival_distr: self.arrival_distr,
            rng: self.rand_gen,
            requests_accum,
            delayed_requests_count: 0,
        }
    }
}

impl<ArrivalDistr, Rand> Pipeline<ArrivalDistr, Rand>
where
    ArrivalDistr: Distribution<Time> + Debug,
    Rand: Rng + Debug,
{
    pub fn reset(&mut self) {
        self.requests_accum.clear();
        self.start_node.reset();
        self.delayed_requests_count = 0;
    }

    pub fn work_during<T: Into<Time>>(&mut self, working_time: T) -> Vec<Request> {
        let working_time = working_time.into();
        let mut counter = Time::ZERO;

        loop {
            let arrival_time = self.arrival_distr.sample(&mut self.rng);

            if counter + arrival_time > working_time {
                break;
            }

            counter += arrival_time;

            let request = Request {
                arrival_time: counter,
                leaving_time: counter,
            };

            if let Err(_) = self.start_node.push_request(&mut arrival_time.clone(), request) {
                self.delayed_requests_count += 1;
            }
        }

        self.requests_accum.to_vec()
    }
}
