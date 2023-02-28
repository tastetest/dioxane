mod pipeline;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    pollster::block_on(pipeline::pipeline::run(event_loop, window));
}
