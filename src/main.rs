use log::LevelFilter;
use renderer::Renderer;
use scene::Scene;
use std::{mem, slice};
use ui::UI;
use winit::{event_loop::EventLoop, window::WindowBuilder};

mod renderer;
mod renderpass;
mod scene;
mod ui;
mod util;

pub type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new().with_title("NEA").build(&event_loop)?;

    let mut renderer = Renderer::new(window);
    let scene = Scene::new(&renderer.window, &renderer.device);
    let ui = UI::new(&renderer, &scene);

    renderer.run(event_loop, scene, ui)?;

    Ok(())
}

pub fn cast_slice<T>(fake: &[T]) -> &[u8] {
    unsafe { slice::from_raw_parts(fake.as_ptr() as _, mem::size_of_val(fake)) }
}