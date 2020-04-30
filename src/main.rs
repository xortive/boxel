use glium::{glutin, Surface};
extern crate nalgebra_glm as glm;

mod primitives;
mod engine;
mod support;

fn main() {

    let event_loop = glutin::event_loop::EventLoop::new();

    let window = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(1024, 768))
        .with_title("Boxel");

    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24) //bits
        .with_vsync(true);


    let mut engine = {
        let display = glium::Display::new(window, context, &event_loop).unwrap();
        engine::Engine::new(display)
    };
   
    event_loop.run(move |ev, _, control_flow| {

        engine.render();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

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
                },
                event::WindowEvent::CursorMoved { position, .. } => {
                    engine.process_cursor((position.x, position.y));
                }, 
                event::WindowEvent::KeyboardInput { input, .. } => {
                    let pressed = input.state == event::ElementState::Pressed;
                    if let Some(key) = input.virtual_keycode {
                        engine.process_keyboard(pressed, key);
                    }
                },
                _ => return,
            },
            _ => (),
        }
    });
}