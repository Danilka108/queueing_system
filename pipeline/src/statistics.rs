use std::ops::{AddAssign, DivAssign};

use crate::{node::PipelineNodeStatistics, Time};

#[derive(Default, Debug)]
pub struct Statistics {
    pub working_time: Time,
    pub requests_number: f32,
    pub handled_requests_number: f32,
    pub delayed_requests_count: f32,
    pub probability_of_request_delay: f32,
    pub average_handling_time: f32,
    pub nodes: Vec<PipelineNodeStatistics>,
}

impl Statistics {
    pub fn get_nodes_idle_time_probabilities(&self) -> Vec<f32> {
        self.nodes
            .iter()
            .map(|n| n.idle_time / f32::from(self.working_time))
            .collect()
    }
}

impl AddAssign<Statistics> for Statistics {
    fn add_assign(&mut self, rhs: Statistics) {
        self.working_time += rhs.working_time;
        self.handled_requests_number += rhs.handled_requests_number;
        self.requests_number += rhs.requests_number;
        self.delayed_requests_count += rhs.delayed_requests_count;
        self.probability_of_request_delay += rhs.probability_of_request_delay;
        self.average_handling_time += rhs.average_handling_time;

        if self.nodes.len() < rhs.nodes.len() {
            let additional_nodes = [PipelineNodeStatistics::default()]
                .into_iter()
                .cycle()
                .take(rhs.nodes.len() - self.nodes.len());

            self.nodes.extend(additional_nodes);
        }

        for i in 0..rhs.nodes.len() {
            self.nodes[i].idle_time += rhs.nodes[i].idle_time;
        }
    }
}

impl DivAssign<f32> for Statistics {
    fn div_assign(&mut self, rhs: f32) {
        self.working_time = Time::from(f32::from(self.working_time) / rhs);
        self.handled_requests_number /= rhs;
        self.requests_number /= rhs;
        self.delayed_requests_count /= rhs;
        self.probability_of_request_delay /= rhs;
        self.average_handling_time /= rhs;

        for i in 0..self.nodes.len() {
            self.nodes[i].idle_time /= rhs;
        }
    }
}
