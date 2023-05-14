use crate::{request::Request, Time};
use rand::Rng;
use std::fmt::Debug;

#[derive(Debug, Default, Clone, Copy)]
pub struct PipelineNodeStatistics {
    pub idle_time: f32,
}

pub trait PipelineNode: Debug {
    fn push_request(&mut self, delta_time: &mut Time, request: Request) -> Result<(), BlockedError>;

    fn reset(&mut self);

    fn get_statistics(&self) -> Vec<PipelineNodeStatistics>;
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
