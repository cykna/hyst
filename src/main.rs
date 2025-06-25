use hyst_engine::{
    HystHandler, HystWindow,
    core::RenderingCore,
    meshes::SizeMethod,
    shaders::events::ShaderEvent,
    ui::{
        HystBoxOptions, HystUi,
        smol_str::SmolStr,
        taffy::{Dimension, Position},
    },
    winit::{event::WindowEvent, window::Window},
};
use hyst_math::vectors::{Rgba, Vec4f32};
pub struct Handler {
    window: Window,
    ui: HystUi,
}

impl HystHandler for Handler {
    fn new(window: Window) -> Self {
        let core = RenderingCore::new(&window);
        let mut ui = HystUi::new(core, Rgba::BLACK);
        ui.create_style(
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
        ui.create_style(
            "suamae",
            hyst_engine::ui::taffy::Style {
                position: Position::Relative,
                size: hyst_engine::ui::taffy::Size {
                    width: Dimension::length(50.0),
                    height: Dimension::percent(50.0),
                },
                ..Default::default()
            },
        );
        ui.create_box(HystBoxOptions {
            bg: hyst_engine::background::Background::Solid(Vec4f32::new(1.0, 1.0, 0.0, 1.0)),
            style: SmolStr::new_inline("suamae"),
        })
        .unwrap();
        ui.create_box(HystBoxOptions {
            bg: hyst_engine::background::Background::Solid(Vec4f32::new(1.0, 0.0, 0.0, 1.0)),
            style: SmolStr::new_inline("seupai"),
        })
        .unwrap();
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
