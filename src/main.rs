use glium::glutin;
use std::env;
extern crate nalgebra_glm as glm;

use std::time::{Duration, Instant};

mod config;
mod engine;
mod primitives;
mod support;

fn main() {
    let mut seed = 148714812; 
    println!("ARGS {}", env::args().len());
    if env::args().len() > 1 {
        seed = env::args().nth(1).unwrap().parse().unwrap();
        println!("new seed {}", seed);
    }

    let event_loop = glutin::event_loop::EventLoop::new();

    let window = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(1024, 768))
        .with_title("Boxel");

    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24); //bits
        // .with_vsync(true);

    let display = glium::Display::new(window, context, &event_loop).unwrap();
    let _scale_factor = display.gl_window().window().scale_factor();

    let mut engine = engine::Engine::new(display, seed);

    let mut last_frame = Instant::now();

    event_loop.run(move |ev, _, control_flow| {

        let now = Instant::now();
        let delta_time = now - last_frame;
        let next_frame_time = now + Duration::from_nanos(16_666_667);
        last_frame = now;

        let fps = 1.0 / delta_time.as_secs_f32();
        if fps < 50.0 {
            println!("FPS: {}/s", 1.0 / delta_time.as_secs_f32());
        }

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        use glium::glutin::{event, event_loop};
        match ev {
            /*
            This match is a little weird...
            so event::Event is an enum struct, of which event::Event::WindowEvent is a possible value
            which has a property event, which is a event::WindowEvent (this is the one we care about for matching on)
            */
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit;
                    return;
                }
                event::WindowEvent::KeyboardInput { input, .. } => {
                    let pressed = input.state == event::ElementState::Pressed;
                    if let Some(key) = input.virtual_keycode {
                        engine.process_keyboard(pressed, key, delta_time);
                    }
                }
                event::WindowEvent::MouseInput { state, button, .. } => {
                    if state == event::ElementState::Pressed && button == event::MouseButton::Left {
                        engine.process_click();
                    }
                }
                _ => return,
            },
            event::Event::DeviceEvent { event, .. } => match event {
                event::DeviceEvent::MouseMotion { delta } => {
                    engine.process_cursor(delta, delta_time);
                }
                _ => return,
            },
            _ => (),
        }
        engine.render();
    });
}
