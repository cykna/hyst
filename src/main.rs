use hyst_engine::{
    HystHandler, HystWindow,
    core::RenderingCore,
    meshes::SizeMethod,
    shaders::events::ShaderEvent,
    ui::{HystBoxOptions, HystImageOptions, HystUi},
    winit::{event::WindowEvent, window::Window},
};
use hyst_math::{
    Rect,
    vectors::{Rgba, Vec2f32, Vec4f32},
};
pub struct Handler {
    window: Window,
    ui: HystUi,
}

impl HystHandler for Handler {
    fn new(window: Window) -> Self {
        let core = RenderingCore::new(&window);
        let mut ui = HystUi::new(core, Rgba::BLACK);
        ui.create_box(HystBoxOptions {
            size_method: SizeMethod::Percentage(0.75, 0.5),
            bg: hyst_engine::background::Background::Solid(Vec4f32::new(1.0, 1.0, 0.0, 1.0)),
            rect: Rect::from_xywh(0.0, 0.0, 0.0, 0.0),
            styles: Vec::new(),
        });
        ui.create_box(HystBoxOptions {
            size_method: SizeMethod::Physical,
            bg: hyst_engine::background::Background::Solid(Vec4f32::new(1.0, 0.0, 0.0, 1.0)),
            rect: Rect::from_xywh(50.0, 50.0, 50.0, 50.0),
            styles: Vec::new(),
        });
        ui.create_image(HystImageOptions {
            rect: Rect::from_xywh(10.0, 10.0, 100.0, 100.0),
            source: "/home/cycro/Pictures/wallpapers/livinda.png".to_string(),
            styles: Vec::new(),
            size_method: SizeMethod::Physical,
        });
        Self { ui, window }
    }
    fn on_window_event(
        &mut self,
        event: hyst_engine::winit::event::WindowEvent,
        id: hyst_engine::winit::window::WindowId,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                self.ui.draw();
            }
            WindowEvent::Resized(size) => {
                size.on_executed(&mut self.ui);
            }
            _ => {}
        }
    }
}

fn main() {
    let mut window = HystWindow::<Handler>::new();
    window.run();
}
