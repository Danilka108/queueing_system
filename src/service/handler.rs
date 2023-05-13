use rand::{prelude::Distribution, Rng};

use super::task::{HandlingState, Task};
use pipeline::{Request, Time};
use std::{collections::VecDeque, ops::ControlFlow};

#[derive(Debug)]
pub(super) struct Handler<R, D>
where
    R: Rng,
    D: Distribution<Time>,
{
    buffer: VecDeque<Request>,
    buffer_size: usize,
    current_task: Option<Task>,
    rng: R,
    distr: D,
}

#[derive(Debug)]
pub(super) struct FullBufferError;

impl<R, D> Handler<R, D>
where
    R: Rng,
    D: Distribution<Time>,
{
    pub fn new(buffer_size: usize, rng: R, distr: D) -> Self {
        Self {
            buffer: VecDeque::with_capacity(buffer_size),
            buffer_size,
            current_task: None,
            rng,
            distr,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.current_task.take();
    }

    pub fn add(&mut self, request: Request) -> Result<(), FullBufferError> {
        if self.buffer.len() == self.buffer_size {
            Err(FullBufferError)
        } else {
            Ok(self.buffer.push_back(request))
        }
    }

    pub fn progress_during(&mut self, delta_time: &mut Time) -> ControlFlow<(), Option<Request>> {
        if *delta_time == Time::ZERO {
            return ControlFlow::Break(());
        }

        let (waiting_time, maybe_request) = match self.get_task()?.handle_during(delta_time) {
            HandlingState::Completed {
                request,
                waiting_time,
            } => (waiting_time, Some(request)),
            HandlingState::InProgress { task, waiting_time } => {
                self.current_task = Some(task);
                (waiting_time, None)
            }
        };

        for i in 0..self.buffer.len() {
            self.buffer[i].leaving_time += waiting_time;
        }

        ControlFlow::Continue(maybe_request)
    }

    fn get_task(&mut self) -> ControlFlow<(), Task> {
        self.current_task
            .take()
            .or_else(|| self.create_task())
            .map_or(ControlFlow::Break(()), |task| ControlFlow::Continue(task))
    }

    fn create_task(&mut self) -> Option<Task> {
        let request = self.buffer.pop_front()?;
        let handling_time = self.distr.sample(&mut self.rng);

        Some(Task {
            request,
            handling_time,
        })
    }
}

impl std::fmt::Display for FullBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "handler buffer full")
    }
}

impl std::error::Error for FullBufferError {}
