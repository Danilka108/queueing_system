use exp_distr::ExpDistr;
use gl_window_provider::GlWindowProvider;
use graph_generator::{achive_calc_accuracy, GraphGenerator};
use pipeline::{PipelineParams, Time};
use rand::thread_rng;
use renderer::GraphRenderer;
use service::ServiceParams;
use winit::event_loop::EventLoop;

use crate::graph_generator::calc_average_stats;

mod exp_distr;
mod graph_generator;
mod renderer;
mod service;

fn main() {
    let mut pipeline = PipelineParams {
        arrival_distr: ExpDistr::new(0.4),
        rand_gen: thread_rng(),
    }
    .build(vec![
        Box::new(ServiceParams {
            buffer_size: 4,
            handling_time_distribution: ExpDistr::new(1.25),
        }),
        Box::new(ServiceParams {
            buffer_size: 2,
            handling_time_distribution: ExpDistr::new(0.5),
        }),
        Box::new(ServiceParams {
            buffer_size: 2,
            handling_time_distribution: ExpDistr::new(0.5),
        }),
    ]);

    pipeline.work_during(Time::from(750.0));
    let stats = pipeline.get_statistics();
    dbg!(&stats);
    dbg!(stats.get_nodes_idle_time_probabilities());
    pipeline.reset();

    let mut generator = GraphGenerator::new(pipeline, false, 750.0, 10.0);

    let n = achive_calc_accuracy(&mut generator, 100.0, 0.2, 1.95);
    dbg!(n);

    // let stats = calc_average_stats(&mut generator, n as usize);
    // dbg!(&stats);
    // dbg!(&stats.get_nodes_idle_time_probabilities());

    let event_loop = EventLoop::new();
    let handler = GlWindowProvider::new(&event_loop).build_handler::<GraphRenderer, ()>();
    event_loop.run(handler);
}
