use winit::{event_loop::{ EventLoop, ControlFlow}, window::{WindowBuilder}, event::{Event, WindowEvent}, dpi::{PhysicalSize}};

pub mod render_engine;

fn main() {
	env_logger::init();
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new().build(&event_loop).expect("unable to create window");
	window.set_inner_size(PhysicalSize::new(1920, 1080));

	let mut renderer = render_engine::render_engine::RenderEngine::new(&window);

	event_loop.run(move |event, _, control_flow| match event {
		Event::RedrawRequested(window_id) if window_id == window.id() => {
			renderer.render();
		}
		Event::MainEventsCleared => {
			window.request_redraw();
		}	
		Event::WindowEvent {
			ref event,
			window_id,
		} if window_id == window.id() => match event {
			WindowEvent::CloseRequested => {*control_flow = ControlFlow::Exit},
			_ => {},		
		}
		_ => {}
	});
}
