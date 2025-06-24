use hyst_engine::{
    HystHandler, HystWindow,
    core::RenderingCore,
    rectangle::Rect,
    rgba::Rgba,
    shaders::events::ShaderEvent,
    ui::HystUi,
    vec4::Vec4f32,
    winit::{event::WindowEvent, window::Window},
};

pub struct Handler {
    window: Window,
    ui: HystUi,
}

impl HystHandler for Handler {
    fn new(window: Window) -> Self {
        let core = RenderingCore::new(&window);
        let mut ui = HystUi::new(core, Rgba::BLACK);
        ui.create_box(hyst_engine::ui::HystBoxCreationOption {
            size_method: hyst_engine::mesh::SizeMethod::Percentage(0.75, 0.5),
            background: hyst_engine::background::Background::Solid(Vec4f32::new(
                1.0, 1.0, 0.0, 1.0,
            )),
            rect: Rect::from_xywh(0.0, 0.0, 0.0, 0.0),
        });
        ui.create_box(hyst_engine::ui::HystBoxCreationOption {
            size_method: hyst_engine::mesh::SizeMethod::Physical,
            background: hyst_engine::background::Background::Solid(Vec4f32::new(
                1.0, 0.0, 0.0, 1.0,
            )),
            rect: Rect::from_xywh(50.0, 50.0, 50.0, 50.0),
        });
        ui.create_image(hyst_engine::ui::HystImageCreationOption {
            rect: Rect::from_xywh(10.0, 10.0, 100.0, 100.0),
            source: "/home/cycro/Pictures/wallpapers/livinda.png".to_string(),
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
