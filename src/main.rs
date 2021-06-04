#[macro_use]
extern crate glium;
extern crate image;

use glium::glutin;
use glium::Surface;
use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex{
    position: [f32; 2],
    tex_coords: [f32; 2]
}

implement_vertex!(Vertex, position, tex_coords);

fn main() {
    

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex1 = Vertex{ position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex{ position: [0.0, 0.5], tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex{ position: [0.5, -0.25], tex_coords: [1.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let image = image::load(Cursor::new(&include_bytes!("../assets/BotW_Link.png")), image::ImageFormat::Png).unwrap().to_rgba8();

    let image_dimensions = image.dimensions();
    
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    // If vec4 * matrix it will rotate in the other direction
    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        uniform mat4 matrix;
        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D tex;
        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = -0.5;

    event_loop.run(move |ev,_, control_flow|{
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _=> return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_67);

        let mut target = display.draw();

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        t += 0.0002;
        if t > 0.5{
            t = -0.5;
        } 

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0],
            ],
            tex: &texture,
        };

        target.clear_color(0.0, 0.0, 1.0, 1.0);
        
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();

        target.finish().unwrap();

    });
}