mod array;
mod error;
mod shader_kind;
mod shader_program;

use std::ffi::CString;

use gl_window_provider::Renderer;
use pipeline::{PipelineParams, Time};
use rand::{prelude::Distribution, thread_rng};

use crate::{exp_distr::ExpDistr, graph_generator::GraphGenerator, service::ServiceParams};

use self::{
    array::VerticesArray,
    shader_program::{ShaderProgram, ShaderProgramBuilder},
};

#[derive(Debug)]
struct TestDistr {
    mean: f32,
}

impl TestDistr {
    fn new(mean: f32) -> Self {
        Self { mean }
    }
}

impl Distribution<Time> for TestDistr {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Time {
        Time::from(self.mean)
    }
}

pub(crate) struct GraphRenderer {
    gl: gl::Gl,
    points_array: VerticesArray,
    program: ShaderProgram,
}

impl Renderer for GraphRenderer {
    fn new<D>(gl_display: &D) -> Self
    where
        D: glutin::prelude::GlDisplay,
    {
        let gl = gl::Gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });

        let pipeline = PipelineParams {
            arrival_distr: ExpDistr::new(0.4),
            rand_gen: thread_rng(),
        }
        .build(vec![
            Box::new(ServiceParams {
                buffer_size: 4,
                handling_time_distribution: TestDistr::new(1.25),
            }),
            Box::new(ServiceParams {
                buffer_size: 2,
                handling_time_distribution: ExpDistr::new(0.5),
            }),
        ]);

        let max_x = 5000.0;
        let (max_y, graph_points) = GraphGenerator::new(pipeline).generate(max_x, 5.0);
        dbg!(graph_points.last());

        let graph_points = graph_points
            .into_iter()
            .map(|(x, y)| (x / max_x, y / max_y))
            .collect::<Vec<_>>();

        let points_array = VerticesArray::new(gl.clone(), graph_points);
        points_array.use_array();

        let program = ShaderProgramBuilder::new(gl.clone())
            .vertex_shader(include_bytes!("./program/vertex_shader.glsl"))
            .fragment_shader(include_bytes!("./program/fragment_shader.glsl"))
            .build()
            .unwrap();

        let x_attrib_pos = program.attrib_location_of("x");
        let y_attrib_pos = program.attrib_location_of("y");

        points_array.set_attrib_pointer(
            x_attrib_pos,
            array::AttribPointer {
                size: array::Size::One,
                stride: 2 * std::mem::size_of::<f32>(),
                offset: 0,
                ty: gl::FLOAT,
            },
            false,
        );

        points_array.set_attrib_pointer(
            y_attrib_pos,
            array::AttribPointer {
                size: array::Size::One,
                stride: 2 * std::mem::size_of::<f32>(),
                offset: std::mem::size_of::<f32>(),
                ty: gl::FLOAT,
            },
            false,
        );

        Self {
            gl,
            points_array,
            program,
        }
    }

    fn draw(&mut self, _: Option<u32>, _: Option<u32>) {
        unsafe {
            self.gl.ClearColor(1.0, 1.0, 1.0, 1.0);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        self.points_array.use_array();
        self.program.use_program();

        unsafe {
            self.gl
                .DrawArrays(gl::LINE_STRIP, 0, self.points_array.len() as i32);
        }
    }

    fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}
