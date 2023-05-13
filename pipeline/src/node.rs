use crate::{request::Request, Time};
use rand::Rng;
use std::fmt::Debug;

pub trait PipelineNode: Debug {
    fn push_request(&mut self, delta_time: &mut Time, request: Request) -> Result<(), BlockedError>;

    fn push_stuck_request(&mut self, request: Request) -> Result<(), BlockedError>;

    fn reset(&mut self);
}

pub trait IntoPipelineNode<R>
where
    R: Rng,
{
    fn into_node(
        self: Box<Self>,
        rand_gen: R,
        next: Box<dyn PipelineNode>,
    ) -> Box<dyn PipelineNode>;
}

#[derive(Debug)]
pub struct BlockedError;

impl std::fmt::Display for BlockedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "node blocked")
    }
}

impl std::error::Error for BlockedError {}
