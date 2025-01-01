use log::LevelFilter;
use renderer::Renderer;
use scene::Scene;
use std::{mem, slice};
use ui::UI;
use winit::{event_loop::EventLoop, window::WindowBuilder};

mod renderer;
mod scene;
mod sim;
mod standardpass;
mod ui;
mod util;

pub type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    std::env::remove_var("WAYLAND_DISPLAY");
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new().with_title("NEA").build(&event_loop)?;

    let mut renderer = Renderer::new(&window);
    let scene = Scene::new(&renderer.window, &renderer.device);
    let ui = UI::new(&renderer, &scene);
    let cascade = sim::Cascade::new(&renderer, &scene.consts);

    renderer.run(event_loop, scene, ui, cascade)?;

    Ok(())
}

pub fn cast_slice<T>(x: &[T]) -> &[u8] {
    unsafe { slice::from_raw_parts(x.as_ptr() as _, mem::size_of_val(x)) }
}
