mod handler;
mod task;

use std::{fmt::Debug, ops::ControlFlow};

use self::handler::Handler;
use pipeline::{
    node::{BlockedError, IntoPipelineNode, PipelineNode, PipelineNodeStatistics},
    Request, Time,
};

use rand::{prelude::Distribution, Rng};

#[derive(Debug)]
pub struct ServiceParams<D>
where
    D: Distribution<Time>,
{
    pub buffer_size: usize,
    pub handling_time_distribution: D,
}

#[derive(Debug)]
pub(crate) struct Service<R, D>
where
    R: Rng,
    D: Distribution<Time>,
{
    state: State,
    idle_time: Time,
    handler: Handler<R, D>,
    next: Box<dyn PipelineNode>,
}

#[derive(Debug)]
enum State {
    Blocked { stuck_request: Request },
    Active,
}

impl<R, D> IntoPipelineNode<R> for ServiceParams<D>
where
    R: Rng + Debug + 'static,
    D: Distribution<Time> + Debug + 'static,
{
    fn into_node(
        self: Box<Self>,
        rand_gen: R,
        next: Box<dyn PipelineNode>,
    ) -> Box<dyn PipelineNode> {
        let handler = Handler::new(self.buffer_size, rand_gen, self.handling_time_distribution);

        Box::new(Service {
            next,
            handler,
            state: State::Active,
            idle_time: Time::ZERO,
        })
    }
}

impl<R, D> PipelineNode for Service<R, D>
where
    R: Rng + Debug,
    D: Distribution<Time> + Debug,
{
    fn push_request(
        &mut self,
        delta_time: &mut Time,
        request: Request,
    ) -> Result<(), BlockedError> {
        if let State::Blocked { stuck_request } = self.state {
            if let Err(_) = self.next.push_request(delta_time, stuck_request) {
                self.idle_time += *delta_time;
                return Err(BlockedError);
            }

            self.state = State::Active;
        }

        let handle_res = self.handle_requests(delta_time);
        self.idle_time += *delta_time;
        handle_res?;

        self.handler.add(request).map_err(|_| BlockedError)
    }

    fn get_statistics(&self) -> Vec<PipelineNodeStatistics> {
        let mut statistics = vec![PipelineNodeStatistics {
            idle_time: f32::from(self.idle_time),
        }];
        statistics.append(&mut self.next.get_statistics());

        statistics
    }

    fn reset(&mut self) {
        self.handler.clear();
        self.next.reset();
        self.idle_time = Time::ZERO;
    }
}

impl<R, D> Service<R, D>
where
    R: Rng,
    D: Distribution<Time>,
{
    fn handle_requests(&mut self, delta_time: &mut Time) -> Result<(), BlockedError> {
        let mut last_delta_time = *delta_time;

        loop {
            let request = match self.handler.progress_during(delta_time) {
                ControlFlow::Continue(Some(request)) => request,
                ControlFlow::Continue(None) => continue,
                ControlFlow::Break(()) => break Ok(()),
            };

            if let err @ Err(BlockedError) = self
                .next
                .push_request(&mut (last_delta_time - *delta_time), request)
            {
                self.state = State::Blocked {
                    stuck_request: request,
                };
                break err;
            }

            last_delta_time = *delta_time;
        }
    }
}
