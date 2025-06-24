use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};
pub trait HystHandler: Sized {
    fn new(window: Window) -> Self;
    fn on_window_event(&mut self, event: WindowEvent, id: WindowId);
    fn finalize(&mut self) {}
}

pub struct HystWindow<H>
where
    H: HystHandler,
{
    handler: Option<H>,
}

impl<H> HystWindow<H>
where
    H: HystHandler,
{
    pub fn new() -> Self {
        Self { handler: None }
    }

    pub fn run(&mut self) {
        let lp = winit::event_loop::EventLoop::new().unwrap();
        lp.run_app(self).unwrap();
    }
}

impl<H> ApplicationHandler for HystWindow<H>
where
    H: HystHandler,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_resizable(true)
                    .with_title("Hyst")
                    .with_transparent(true),
            )
            .unwrap();
        self.handler = Some(H::new(window));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let flag = matches!(event, WindowEvent::CloseRequested);
        if let Some(ref mut handler) = self.handler {
            handler.on_window_event(event, id);
        }
        if flag {
            event_loop.exit();
            self.handler = None;
        }
    }
}
