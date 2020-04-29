use glium::{glutin, Surface};

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

    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let mut engine = engine::Engine::new(&display);
    

    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color_and_depth((0.529, 0.808, 0.980, 1.0), 1.0);
        engine.render(&mut target);
        target.finish().unwrap();

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