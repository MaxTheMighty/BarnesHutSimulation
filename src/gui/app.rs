use crate::gui::state::State;
use std::sync::Arc;
use cgmath::{ElementWise, Point3};
use log::info;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalPosition;
use winit::event::{DeviceId, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

// Container for our application, with special cfg's for web assembly
pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        Self {
            state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
        }
    }

}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.state = Some(pollster::block_on(State::new(window)).unwrap());
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        // This is where proxy.send_event() ends up
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let app_state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CursorMoved {
                device_id: id,
                position,
            } => {
               
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => app_state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                app_state.update();
                match app_state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = app_state.window.inner_size();
                        app_state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => match (code, state.is_pressed()) {
                (KeyCode::Escape, true) => event_loop.exit(),
                (KeyCode::Space, true) => {
                    app_state.gpu.increment_texture();
                },
                // (KeyCode::ArrowUp, true) => {
                //     self.state.as_mut().unwrap().gpu.camera.eye.add_assign_element_wise(1.0f32);
                //     log::info!("Up pressed, camera eye is now {:?}",self.state.as_ref().unwrap().gpu.camera.eye);
                // },
                // (KeyCode::ArrowDown, true) => {
                //     self.state.as_mut().unwrap().gpu.camera.eye.add_assign_element_wise(-1.0f32);
                //     log::info!("Down pressed, camera eye is now {:?}",self.state.as_ref().unwrap().gpu.camera.eye);
                // }
                (_, true) => {
                   app_state.gpu.camera_controller.handle_key_code(&mut app_state.gpu.camera, code);
                },
                (_, false) => {}
            },
            _ => {}
        }
    }
}
