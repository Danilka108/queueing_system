mod array;
mod error;
mod shader_kind;
mod shader_program;

use std::ffi::CString;

use gl_window_provider::Renderer;
use pipeline::{PipelineParams, Time};
use rand::{prelude::Distribution, thread_rng};

use crate::{
    exp_distr::ExpDistr,
    graph_generator::{Graph, GraphGenerator},
    service::ServiceParams,
};

use self::{
    array::VerticesArray,
    shader_program::{ShaderProgram, ShaderProgramBuilder},
};

pub(crate) struct GraphRenderer {
    gl: gl::Gl,
    graph_points_array: VerticesArray,
    lines_points_array: VerticesArray,
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
                handling_time_distribution: ExpDistr::new(1.25),
            }),
            Box::new(ServiceParams {
                buffer_size: 2,
                handling_time_distribution: ExpDistr::new(0.5),
            }),
        ]);

        let max_x = 10000.0;
        let Graph {
            max_x,
            max_y,
            mean,
            deviation,
            points,
        } = GraphGenerator::new(pipeline, true, max_x, 10.0).generate();

        dbg!(mean, deviation);

        let graph_points = points
            .into_iter()
            .map(|(x, y)| (x / max_x, y / max_y, 1.0f32, 0.65f32, 0.0f32))
            .collect::<Vec<_>>();
        let graph_points_array = VerticesArray::new(gl.clone(), &graph_points);

        let lines_points: &[(f32, f32, f32, f32, f32)] = &[
            (0.0, mean / max_y, 0.0, 0.65, 1.0),
            (1.0, mean / max_y, 0.0, 0.65, 1.0),
            (0.0, (mean + deviation) / max_y, 0.0, 0.65, 1.0),
            (1.0, (mean + deviation) / max_y, 0.0, 0.65, 1.0),
            (0.0, (mean - deviation) / max_y, 0.0, 0.65, 1.0),
            (1.0, (mean - deviation) / max_y, 0.0, 0.65, 1.0),
            (0.0, 0.0, 1.0, 0.0, 0.0),
            (1.0, 0.0, 1.0, 0.0, 0.0),
            (0.0, 0.0, 1.0, 0.0, 0.0),
            (0.0, 1.0, 1.0, 0.0, 0.0),
        ];

        let lines_points_array = VerticesArray::new(gl.clone(), lines_points);

        let program = ShaderProgramBuilder::new(gl.clone())
            .vertex_shader(include_bytes!("./program/vertex_shader.glsl"))
            .fragment_shader(include_bytes!("./program/fragment_shader.glsl"))
            .build()
            .unwrap();

        let x_attrib_pos = program.attrib_location_of("iX");
        let y_attrib_pos = program.attrib_location_of("iY");
        let color_attrib_pos = program.attrib_location_of("iColor");

        graph_points_array.use_array();

        graph_points_array.set_attrib_pointer(
            x_attrib_pos,
            array::AttribPointer {
                size: array::Size::One,
                stride: 5 * std::mem::size_of::<f32>(),
                offset: 0,
                ty: gl::FLOAT,
            },
            false,
        );

        graph_points_array.set_attrib_pointer(
            y_attrib_pos,
            array::AttribPointer {
                size: array::Size::One,
                stride: 5 * std::mem::size_of::<f32>(),
                offset: std::mem::size_of::<f32>(),
                ty: gl::FLOAT,
            },
            false,
        );

        graph_points_array.set_attrib_pointer(
            color_attrib_pos,
            array::AttribPointer {
                size: array::Size::Three,
                stride: 5 * std::mem::size_of::<f32>(),
                offset: 2 * std::mem::size_of::<f32>(),
                ty: gl::FLOAT,
            },
            false,
        );

        lines_points_array.use_array();

        lines_points_array.set_attrib_pointer(
            x_attrib_pos,
            array::AttribPointer {
                size: array::Size::One,
                stride: 5 * std::mem::size_of::<f32>(),
                offset: 0,
                ty: gl::FLOAT,
            },
            false,
        );

        lines_points_array.set_attrib_pointer(
            y_attrib_pos,
            array::AttribPointer {
                size: array::Size::One,
                stride: 5 * std::mem::size_of::<f32>(),
                offset: std::mem::size_of::<f32>(),
                ty: gl::FLOAT,
            },
            false,
        );

        lines_points_array.set_attrib_pointer(
            color_attrib_pos,
            array::AttribPointer {
                size: array::Size::Three,
                stride: 5 * std::mem::size_of::<f32>(),
                offset: 2 * std::mem::size_of::<f32>(),
                ty: gl::FLOAT,
            },
            false,
        );

        Self {
            gl,
            graph_points_array,
            lines_points_array,
            program,
        }
    }

    fn draw(&mut self, _: Option<u32>, _: Option<u32>) {
        unsafe {
            self.gl.ClearColor(1.0, 1.0, 1.0, 1.0);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        self.program.use_program();

        self.graph_points_array.use_array();
        unsafe {
            self.gl
                .DrawArrays(gl::LINE_STRIP, 0, self.graph_points_array.len() as i32);
        }

        self.lines_points_array.use_array();
        unsafe {
            self.gl
                .DrawArrays(gl::LINES, 0, self.lines_points_array.len() as i32);
        }
    }

    fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}
