use hyst_engine::{
    HystHandler, HystWindow,
    core::RenderingCore,
    shaders::events::ShaderEvent,
    ui::{
        HystUi,
        pulse::Pulse,
        taffy::{Dimension, Position},
    },
    winit::{event::WindowEvent, window::Window},
};
use hyst_math::vectors::{Rgba, Vec2f32, Vec4f32};
pub struct Handler {
    window: Window,
    ui: HystUi,
    text: Pulse<String>,
}

impl HystHandler for Handler {
    fn new(window: Window) -> Self {
        let core = RenderingCore::new(&window);
        let mut ui = HystUi::new(core, Rgba::BLACK);
        ui.create_layout(
            "seupai",
            hyst_engine::ui::taffy::Style {
                position: Position::Relative,
                size: hyst_engine::ui::taffy::Size {
                    width: Dimension::percent(0.5),
                    height: Dimension::percent(0.25),
                },
                ..Default::default()
            },
        );
        ui.create_layout(
            "suamae",
            hyst_engine::ui::taffy::Style {
                position: Position::Relative,
                size: hyst_engine::ui::taffy::Size {
                    width: Dimension::length(50.0),
                    height: Dimension::percent(0.5),
                },
                ..Default::default()
            },
        );
        let text = ui.create_pulse(String::from("Jorge"));
        ui.create_text(hyst_engine::ui::HystTextOptions {
            content: text.clone(),
            position: Vec2f32::new(80.0, 80.0),
            style: "suamae".into(),
            font_size: 25.0,
            color: Vec4f32::new(1.0, 0.0, 0.0, 1.0),
        })
        .unwrap();
        ui.create_text(hyst_engine::ui::HystTextOptions {
            content: text.clone(),
            position: Vec2f32::new(40.0, 0.0),
            style: "suamae".into(),
            font_size: 12.0,
            color: Vec4f32::new(0.0, 1.0, 0.0, 0.5),
        })
        .unwrap();

        Self { ui, window, text }
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
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                self.text.mutate(|mut t| t.push('e'));
            }
            _ => {}
        }
        if self.ui.check_for_updates() {
            self.window.request_redraw();
        };
    }
}

fn main() {
    let mut window = HystWindow::<Handler>::new();
    window.run();
}
