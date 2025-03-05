use engine::Engine;
use log::LevelFilter;
use std::{mem, slice};
use winit::{event_loop::EventLoop, window::WindowBuilder};

mod engine;
mod renderer;
mod scene;
mod sim;
mod simulation;
mod ui;
mod util;

pub type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const WG_SIZE: u32 = 8;

fn main() -> Result {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    std::env::remove_var("WAYLAND_DISPLAY");
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new().with_title("NEA").build(&event_loop)?;

    let mut engine = Engine::new(&window);

    engine.run(event_loop)?;
    Ok(())
}

pub fn cast_slice<T>(x: &[T]) -> &[u8] {
    unsafe { slice::from_raw_parts(x.as_ptr() as _, mem::size_of_val(x)) }
}
