use gl_window_provider::GlWindowProvider;
use renderer::GraphRenderer;
use winit::event_loop::EventLoop;

mod exp_distr;
mod graph_generator;
mod renderer;
mod service;

fn main() {
    let event_loop = EventLoop::new();
    let handler = GlWindowProvider::new(&event_loop).build_handler::<GraphRenderer, ()>();
    event_loop.run(handler);
}
