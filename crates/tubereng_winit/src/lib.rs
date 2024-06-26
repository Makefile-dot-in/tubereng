#![warn(clippy::pedantic)]

use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

use tubereng_engine::Engine;
use tubereng_input::{keyboard::Key, mouse::Button, Input};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    error::{EventLoopError, OsError},
    event::{DeviceEvent, Event, KeyEvent, MouseButton, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

#[derive(Debug)]
pub enum WinitError {
    EventLoopCreationFailed(EventLoopError),
    EventLoopRunningFailed(EventLoopError),
    WindowCreationFailed(OsError),
    WindowHandleFetchingFailed(raw_window_handle::HandleError),
}

pub struct WinitTuberRunner;
impl WinitTuberRunner {
    /// Starts the application using a winit window.
    ///
    /// # Errors
    ///
    /// Will return [`Err`] if the event loop cannot be created or run, or if
    /// the window cannot be created.
    ///
    /// # Panics
    ///
    /// For wasm32, might panic if the window canvas cannot be added to the page.
    pub async fn run(mut engine: Engine) -> Result<(), WinitError> {
        let event_loop = EventLoop::new().map_err(WinitError::EventLoopCreationFailed)?;
        let window = Arc::new(
            WindowBuilder::new()
                .with_title(engine.application_title())
                .with_resizable(false)
                .with_inner_size(PhysicalSize::new(800, 600))
                .build(&event_loop)
                .map_err(WinitError::WindowCreationFailed)?,
        );
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            let _ = window.request_inner_size(PhysicalSize::new(800, 600));

            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id(engine.application_title())?;
                    let canvas = web_sys::Element::from(window.canvas()?);
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }
        engine.init_graphics(window.clone()).await;
        let mut last_frame_start_instant = Instant::now();
        event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    elwt.exit();
                }
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => engine.on_input(Input::MouseMotion(delta)),
                Event::WindowEvent {
                    event:
                        WindowEvent::CursorMoved {
                            position: PhysicalPosition { x, y },
                            ..
                        },
                    ..
                } => engine.on_input(Input::CursorMoved((x, y))),
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    window.request_redraw();
                    let frame_start_instant = Instant::now();
                    let delta_time = (frame_start_instant - last_frame_start_instant).as_secs_f32();
                    engine.update(delta_time);
                    last_frame_start_instant = frame_start_instant;
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { state, button, .. },
                    ..
                } => match state {
                    winit::event::ElementState::Pressed => {
                        engine.on_input(Input::MouseButtonDown(WinitButton(button).into()));
                    }
                    winit::event::ElementState::Released => {
                        engine.on_input(Input::MouseButtonUp(WinitButton(button).into()));
                    }
                },
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state,
                                    physical_key: PhysicalKey::Code(virtual_keycode),
                                    ..
                                },
                            ..
                        },
                    ..
                } => match state {
                    winit::event::ElementState::Pressed => {
                        engine.on_input(Input::KeyDown(WinitKeyCode(virtual_keycode).into()));
                    }

                    winit::event::ElementState::Released => {
                        engine.on_input(Input::KeyUp(WinitKeyCode(virtual_keycode).into()));
                    }
                },
                _ => {}
            })
            .map_err(WinitError::EventLoopRunningFailed)?;

        Ok(())
    }
}

struct WinitButton(MouseButton);
impl From<WinitButton> for Button {
    fn from(value: WinitButton) -> Self {
        let button = value.0;
        match button {
            MouseButton::Left => Button::Left,
            MouseButton::Middle => Button::Middle,
            MouseButton::Right => Button::Right,
            _ => Button::Unknown,
        }
    }
}

struct WinitKeyCode(KeyCode);
impl From<WinitKeyCode> for Key {
    fn from(value: WinitKeyCode) -> Self {
        let virtual_key_code = value.0;
        match virtual_key_code {
            KeyCode::Escape => Key::Escape,
            KeyCode::Space => Key::Space,
            KeyCode::ArrowUp => Key::ArrowUp,
            KeyCode::ArrowDown => Key::ArrowDown,
            KeyCode::ArrowLeft => Key::ArrowLeft,
            KeyCode::ArrowRight => Key::ArrowRight,
            KeyCode::KeyA => Key::A,
            KeyCode::KeyB => Key::B,
            KeyCode::KeyC => Key::C,
            KeyCode::KeyD => Key::D,
            KeyCode::KeyE => Key::E,
            KeyCode::KeyF => Key::F,
            KeyCode::KeyG => Key::G,
            KeyCode::KeyH => Key::H,
            KeyCode::KeyI => Key::I,
            KeyCode::KeyJ => Key::J,
            KeyCode::KeyK => Key::K,
            KeyCode::KeyL => Key::L,
            KeyCode::KeyM => Key::M,
            KeyCode::KeyN => Key::N,
            KeyCode::KeyO => Key::O,
            KeyCode::KeyP => Key::P,
            KeyCode::KeyQ => Key::Q,
            KeyCode::KeyR => Key::R,
            KeyCode::KeyS => Key::S,
            KeyCode::KeyT => Key::T,
            KeyCode::KeyU => Key::U,
            KeyCode::KeyV => Key::V,
            KeyCode::KeyW => Key::W,
            KeyCode::KeyX => Key::X,
            KeyCode::KeyY => Key::Y,
            KeyCode::KeyZ => Key::Z,
            _ => Key::Unknown,
        }
    }
}
