// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(plugin, custom_attribute)]
#![plugin(gfx_macros)]

extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::*;

#[vertex_format]
#[derive(Clone, Copy)]
struct Vertex {
    #[name = "a_Pos"]
    pos: [f32; 2],

    #[name = "a_Color"]
    color: [f32; 3],
}

pub fn main() {
    let mut canvas = gfx_window_glutin::init(glutin::Window::new().unwrap())
                                       .into_canvas();
    canvas.output.window.set_title("Triangle example");

    let vertex_data = [
        Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
        Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
        Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] },
    ];
    let mesh = canvas.factory.create_mesh(&vertex_data);
    let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);

    let program = {
        let vs = gfx::ShaderSource {
            glsl_120: Some(include_bytes!("triangle_120.glslv")),
            glsl_150: Some(include_bytes!("triangle_150.glslv")),
            .. gfx::ShaderSource::empty()
        };
        let fs = gfx::ShaderSource {
            glsl_120: Some(include_bytes!("triangle_120.glslf")),
            glsl_150: Some(include_bytes!("triangle_150.glslf")),
            .. gfx::ShaderSource::empty()
        };
        canvas.factory.link_program_source(vs, fs, &canvas.device.get_capabilities())
                      .unwrap()
    };
    let state = gfx::DrawState::new();

    'main: loop {
        // quit when Esc is pressed.
        for event in canvas.output.window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) => break 'main,
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }

        canvas.clear(gfx::ClearData {
            color: [0.3, 0.3, 0.3, 1.0],
            depth: 1.0,
            stencil: 0,
        });
        canvas.draw(&gfx::batch::bind(&state, &mesh, slice.clone(), &program, &None))
              .unwrap();
        canvas.present();
    }
}
