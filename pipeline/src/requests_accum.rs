use crate::node::{BlockedError, PipelineNode, PipelineNodeStatistics};
use crate::request::Request;
use crate::Time;
use std::slice::Iter;
use std::{cell::RefCell, rc::Rc};

pub(crate) fn requests_accum<'node>() -> (RequestsAccum, Box<dyn PipelineNode + 'node>) {
    let requests = Default::default();

    (
        RequestsAccum(Rc::clone(&requests)),
        Box::new(RequestsAccumNode(requests)),
    )
}

#[derive(Debug)]
pub(crate) struct RequestsAccum(Rc<RefCell<Vec<Request>>>);

impl RequestsAccum {
    pub fn clear(&mut self) {
        self.0.borrow_mut().clear();
    }

    pub fn to_vec(&self) -> Vec<Request> {
        self.0.borrow().iter().copied().collect()
    }

    pub fn iter(&self) -> std::vec::IntoIter<Request> {
        self.0.borrow().clone().into_iter()
    }
}

struct RequestsAccumNode(Rc<RefCell<Vec<Request>>>);

impl std::fmt::Debug for RequestsAccumNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequestsAccumNode {{ .. }}")
    }
}

impl PipelineNode for RequestsAccumNode {
    fn push_request(&mut self, _: &mut Time, request: Request) -> Result<(), BlockedError> {
        (*self.0).borrow_mut().push(request);
        Ok(())
    }

    fn get_statistics(&self) -> Vec<PipelineNodeStatistics> {
        Vec::new()
    }

    fn reset(&mut self) {}
}
