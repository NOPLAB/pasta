use std::{collections::HashMap, sync::Arc};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{self, ActiveEventLoop},
    window::{Window, WindowId},
};

struct ApplicationWindow {
    window: Arc<Window>,
}

impl ApplicationWindow {
    fn new(window: Window) -> Self {
        let window = Arc::new(window);

        Self { window }
    }
}

pub struct Application {
    windows: HashMap<WindowId, ApplicationWindow>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn create_window(&mut self, event_loop: &ActiveEventLoop) -> anyhow::Result<WindowId> {
        let window_attrs = Window::default_attributes().with_title("window");

        let window = event_loop.create_window(window_attrs)?;

        let window_state = ApplicationWindow::new(window);
        let window_id = window_state.window.id();

        self.windows.insert(window_state.window.id(), window_state);

        Ok(window_id)
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        log::info!("Application resumed");

        self.create_window(event_loop).unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {}
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {}
}
