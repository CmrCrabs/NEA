use std::mem;

use crate::{
    cast_slice,
    renderer::{Renderer, FORMAT},
    scene::Scene,
    util::Texture,
};
use imgui::{BackendFlags, DrawVert, FontSource, Key, MouseCursor, TreeNodeFlags, Ui};
use shared::Constants;
use wgpu::{util::DeviceExt, BindGroup, Buffer, Device, Queue, RenderPipeline};
use winit::{
    event::{MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorIcon, Window},
};

pub struct UI {
    pub pipeline: RenderPipeline,
    pub vtx_buf: Buffer,
    pub idx_buf: Buffer,
    pub scene_bind_group: BindGroup,
    pub scene_buf: Buffer,
    texture: Texture,
    pub context: imgui::Context,
    pub focused: bool,
}

impl UI {
    pub fn new(renderer: &Renderer, scene: &Scene) -> Self {
        let mut context = imgui::Context::create();
        context.set_ini_filename(None);

        let hidpi_factor = renderer.window.scale_factor();
        let dimensions = renderer.window.inner_size();

        let io = context.io_mut();
        io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
        io.backend_flags.insert(BackendFlags::HAS_SET_MOUSE_POS);
        io.backend_flags
            .insert(BackendFlags::RENDERER_HAS_VTX_OFFSET);
        io.display_size = [dimensions.width as _, dimensions.height as _];
        io.display_framebuffer_scale = [hidpi_factor as f32, hidpi_factor as f32];

        let style = context.style_mut();
        style.window_rounding = 4.0;
        style.popup_rounding = 4.0;
        style.frame_rounding = 2.0;
        style.scale_all_sizes(hidpi_factor as _);

        let fonts = context.fonts();
        fonts.add_font(&[FontSource::DefaultFontData { config: None }]);
        let font_texture = fonts.build_rgba32_texture();
        let texture = Texture::new(
            font_texture.width,
            font_texture.height,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            &renderer,
        );
        texture.write(&renderer.queue, font_texture.data, 4);

        let pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&scene.scene_layout, &renderer.tex_layout],
                    push_constant_ranges: &[],
                });
        let pipeline = renderer
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &renderer.shader,
                    entry_point: Some("ui_vs"),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<DrawVert>() as _,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Unorm8x4],
                    }],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &renderer.shader,
                    entry_point: Some("ui_fs"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: FORMAT,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
                label: None,
            });
        let vtx_buf = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            size: 0,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
            label: None,
        });
        let idx_buf = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            size: 0,
            usage: wgpu::BufferUsages::INDEX,
            mapped_at_creation: false,
            label: None,
        });

        let scene_buf = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            size: scene.mem_size as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            label: None,
        });
        let scene_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &scene.scene_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: scene_buf.as_entire_binding(),
                }],
                label: None,
            });
        let focused = true;

        Self {
            pipeline,
            vtx_buf,
            idx_buf,
            context,
            scene_bind_group,
            scene_buf,
            texture,
            focused,
        }
    }

    pub fn render<'a>(
        &'a mut self,
        device: &Device,
        queue: &Queue,
        encoder: &'a mut wgpu::CommandEncoder,
        surface_view: &'a wgpu::TextureView,
        scene: &Scene,
    ) {
        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            label: None,
        });

        let draw_data = self.context.render();
        if draw_data.total_idx_count == 0 {
            return;
        }
        let mut vertices = Vec::with_capacity(draw_data.total_vtx_count as _);
        let mut indices = Vec::with_capacity(draw_data.total_idx_count as _);
        for draw_list in draw_data.draw_lists() {
            vertices.extend_from_slice(draw_list.vtx_buffer());
            indices.extend_from_slice(draw_list.idx_buffer());
        }

        indices.resize(
            indices.len() + wgpu::COPY_BUFFER_ALIGNMENT as usize
                - indices.len() % wgpu::COPY_BUFFER_ALIGNMENT as usize,
            0,
        );

        if (self.idx_buf.size() as usize) < indices.len() * mem::size_of::<u16>() {
            self.idx_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                contents: cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                label: None,
            });
        } else {
            queue.write_buffer(&self.idx_buf, 0, cast_slice(&indices));
        }

        if (self.vtx_buf.size() as usize) < vertices.len() * mem::size_of::<DrawVert>() {
            self.vtx_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                contents: cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                label: None,
            });
        } else {
            queue.write_buffer(&self.vtx_buf, 0, cast_slice(&vertices));
        }

        queue.write_buffer(&self.scene_buf, 0, cast_slice(&[scene.consts]));
        renderpass.set_pipeline(&self.pipeline);
        renderpass.set_bind_group(0, &self.scene_bind_group, &[]);
        renderpass.set_bind_group(1, &self.texture.bind_group, &[]);
        renderpass.set_vertex_buffer(0, self.vtx_buf.slice(..));
        renderpass.set_index_buffer(self.idx_buf.slice(..), wgpu::IndexFormat::Uint16);

        let mut vtx_offset = 0;
        let mut idx_offset = 0;
        for draw_list in draw_data.draw_lists() {
            for cmd in draw_list.commands() {
                if let imgui::DrawCmd::Elements { count, cmd_params } = cmd {
                    renderpass.set_scissor_rect(
                        cmd_params.clip_rect[0].floor() as _,
                        cmd_params.clip_rect[1].floor() as _,
                        (cmd_params.clip_rect[2] - cmd_params.clip_rect[0].ceil()) as _,
                        (cmd_params.clip_rect[3] - cmd_params.clip_rect[1].ceil()) as _,
                    );
                    let start = idx_offset as u32 + cmd_params.idx_offset as u32;
                    renderpass.draw_indexed(
                        start..(start + count as u32),
                        vtx_offset as i32 + cmd_params.vtx_offset as i32,
                        0..1,
                    );
                }
            }
            vtx_offset += draw_list.vtx_buffer().len();
            idx_offset += draw_list.idx_buffer().len();
        }
        drop(renderpass);
    }

    pub fn update_cursor(&mut self, window: &Window) {
        if let Some(cursor) = self.context.mouse_cursor() {
            window.set_cursor_visible(true);
            window.set_cursor_icon(to_winit_cursor(cursor));
        } else {
            window.set_cursor_visible(false);
        }
    }

    pub fn handle_events(&mut self, event: &WindowEvent) {
        let io = self.context.io_mut();
        match event {
            WindowEvent::Resized(size) => {
                io.display_size = [size.width as _, size.height as _];
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    for k in to_imgui_keys(key) {
                        io.add_key_event(*k, event.state.is_pressed());
                    }
                }
                if event.state.is_pressed() {
                    if let Some(txt) = &event.text {
                        for ch in txt.chars() {
                            io.add_input_character(ch);
                        }
                    }
                }
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                io.display_framebuffer_scale = [*scale_factor as _; 2];
            }
            WindowEvent::MouseInput { state, button, .. } => match button {
                MouseButton::Left => io.mouse_down[0] = state.is_pressed(),
                MouseButton::Right => io.mouse_down[1] = state.is_pressed(),
                MouseButton::Middle => io.mouse_down[2] = state.is_pressed(),
                _ => {}
            },
            WindowEvent::CursorMoved { position, .. } => {
                io.mouse_pos = [position.x as _, position.y as _];
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let sf = 0.01;
                let (h, v) = match delta {
                    MouseScrollDelta::LineDelta(h, v) => (*h, *v),
                    MouseScrollDelta::PixelDelta(pos) => (sf * pos.x as f32, sf * pos.y as f32),
                };
                io.mouse_wheel_h = h;
                io.mouse_wheel = v;
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                io.key_shift = modifiers.state().shift_key();
                io.key_alt = modifiers.state().alt_key();
                io.key_ctrl = modifiers.state().control_key();
                io.key_super = modifiers.state().super_key();
            }
            _ => {}
        }
    }
}

pub fn build(ui: &Ui, consts: &mut Constants) -> bool {
    let mut focused = false;
    ui.window("NEA Ocean Simulation").always_auto_resize(true).build(|| {
        ui.text("Info");
        ui.text(format!("{:.1$} Elapsed", consts.time, 2));
        ui.text(format!("{:.1$} fps", 1.0 / consts.frametime, 0));
        ui.separator();
        ui.text("Simulation Parameters");
        ui.separator();
        ui.text("Shader Parameters");
        ui.color_picker4("Base Color", consts.shader.base_color.as_mut());
        focused = ui.is_window_focused();
    });
    focused
}

fn to_winit_cursor(cursor: MouseCursor) -> CursorIcon {
    match cursor {
        MouseCursor::Arrow => CursorIcon::Default,
        MouseCursor::TextInput => CursorIcon::Text,
        MouseCursor::ResizeAll => CursorIcon::Move,
        MouseCursor::ResizeNS => CursorIcon::NsResize,
        MouseCursor::ResizeEW => CursorIcon::EwResize,
        MouseCursor::ResizeNESW => CursorIcon::NeswResize,
        MouseCursor::ResizeNWSE => CursorIcon::NwseResize,
        MouseCursor::Hand => CursorIcon::Grab,
        MouseCursor::NotAllowed => CursorIcon::NotAllowed,
    }
}

fn to_imgui_keys(keycode: KeyCode) -> &'static [Key] {
    match keycode {
        KeyCode::Tab => &[Key::Tab],
        KeyCode::ArrowLeft => &[Key::LeftArrow],
        KeyCode::ArrowRight => &[Key::RightArrow],
        KeyCode::ArrowUp => &[Key::UpArrow],
        KeyCode::ArrowDown => &[Key::DownArrow],
        KeyCode::PageUp => &[Key::PageUp],
        KeyCode::PageDown => &[Key::PageDown],
        KeyCode::Home => &[Key::Home],
        KeyCode::End => &[Key::End],
        KeyCode::Insert => &[Key::Insert],
        KeyCode::Delete => &[Key::Delete],
        KeyCode::Backspace => &[Key::Backspace],
        KeyCode::Space => &[Key::Space],
        KeyCode::Enter => &[Key::Enter],
        KeyCode::Escape => &[Key::Escape],
        KeyCode::ControlLeft => &[Key::LeftCtrl, Key::ModCtrl],
        KeyCode::ShiftLeft => &[Key::LeftShift, Key::ModShift],
        KeyCode::AltLeft => &[Key::LeftAlt, Key::ModAlt],
        KeyCode::SuperLeft => &[Key::LeftSuper, Key::ModSuper],
        KeyCode::ControlRight => &[Key::RightCtrl, Key::ModCtrl],
        KeyCode::ShiftRight => &[Key::RightShift, Key::ModShift],
        KeyCode::AltRight => &[Key::RightAlt, Key::ModAlt],
        KeyCode::SuperRight => &[Key::RightSuper, Key::ModSuper],
        KeyCode::ContextMenu => &[Key::Menu],
        KeyCode::Digit0 => &[Key::Alpha0],
        KeyCode::Digit1 => &[Key::Alpha1],
        KeyCode::Digit2 => &[Key::Alpha2],
        KeyCode::Digit3 => &[Key::Alpha3],
        KeyCode::Digit4 => &[Key::Alpha4],
        KeyCode::Digit5 => &[Key::Alpha5],
        KeyCode::Digit6 => &[Key::Alpha6],
        KeyCode::Digit7 => &[Key::Alpha7],
        KeyCode::Digit8 => &[Key::Alpha8],
        KeyCode::Digit9 => &[Key::Alpha9],
        KeyCode::KeyA => &[Key::A],
        KeyCode::KeyB => &[Key::B],
        KeyCode::KeyC => &[Key::C],
        KeyCode::KeyD => &[Key::D],
        KeyCode::KeyE => &[Key::E],
        KeyCode::KeyF => &[Key::F],
        KeyCode::KeyG => &[Key::G],
        KeyCode::KeyH => &[Key::H],
        KeyCode::KeyI => &[Key::I],
        KeyCode::KeyJ => &[Key::J],
        KeyCode::KeyK => &[Key::K],
        KeyCode::KeyL => &[Key::L],
        KeyCode::KeyM => &[Key::M],
        KeyCode::KeyN => &[Key::N],
        KeyCode::KeyO => &[Key::O],
        KeyCode::KeyP => &[Key::P],
        KeyCode::KeyQ => &[Key::Q],
        KeyCode::KeyR => &[Key::R],
        KeyCode::KeyS => &[Key::S],
        KeyCode::KeyT => &[Key::T],
        KeyCode::KeyU => &[Key::U],
        KeyCode::KeyV => &[Key::V],
        KeyCode::KeyW => &[Key::W],
        KeyCode::KeyX => &[Key::X],
        KeyCode::KeyY => &[Key::Y],
        KeyCode::KeyZ => &[Key::Z],
        KeyCode::F1 => &[Key::F1],
        KeyCode::F2 => &[Key::F2],
        KeyCode::F3 => &[Key::F3],
        KeyCode::F4 => &[Key::F4],
        KeyCode::F5 => &[Key::F5],
        KeyCode::F6 => &[Key::F6],
        KeyCode::F7 => &[Key::F7],
        KeyCode::F8 => &[Key::F8],
        KeyCode::F9 => &[Key::F9],
        KeyCode::F10 => &[Key::F10],
        KeyCode::F11 => &[Key::F11],
        KeyCode::F12 => &[Key::F12],
        KeyCode::Quote => &[Key::Apostrophe],
        KeyCode::Comma => &[Key::Comma],
        KeyCode::Minus => &[Key::Minus],
        KeyCode::Period => &[Key::Period],
        KeyCode::Slash => &[Key::Slash],
        KeyCode::Semicolon => &[Key::Semicolon],
        KeyCode::Equal => &[Key::Equal],
        KeyCode::BracketLeft => &[Key::LeftBracket],
        KeyCode::Backslash => &[Key::Backslash],
        KeyCode::BracketRight => &[Key::RightBracket],
        KeyCode::Backquote => &[Key::GraveAccent],
        KeyCode::CapsLock => &[Key::CapsLock],
        KeyCode::ScrollLock => &[Key::ScrollLock],
        KeyCode::NumLock => &[Key::NumLock],
        KeyCode::PrintScreen => &[Key::PrintScreen],
        KeyCode::Pause => &[Key::Pause],
        KeyCode::Numpad0 => &[Key::Keypad0],
        KeyCode::Numpad1 => &[Key::Keypad1],
        KeyCode::Numpad2 => &[Key::Keypad2],
        KeyCode::Numpad3 => &[Key::Keypad3],
        KeyCode::Numpad4 => &[Key::Keypad4],
        KeyCode::Numpad5 => &[Key::Keypad5],
        KeyCode::Numpad6 => &[Key::Keypad6],
        KeyCode::Numpad7 => &[Key::Keypad7],
        KeyCode::Numpad8 => &[Key::Keypad8],
        KeyCode::Numpad9 => &[Key::Keypad9],
        KeyCode::NumpadDecimal => &[Key::KeypadDecimal],
        KeyCode::NumpadDivide => &[Key::KeypadDivide],
        KeyCode::NumpadMultiply => &[Key::KeypadMultiply],
        KeyCode::NumpadSubtract => &[Key::KeypadSubtract],
        KeyCode::NumpadAdd => &[Key::KeypadAdd],
        KeyCode::NumpadEnter => &[Key::KeypadEnter],
        KeyCode::NumpadEqual => &[Key::KeypadEqual],
        _ => &[],
    }
}
