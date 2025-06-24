//Event shaders. So, a shader can execute something when an specific event is executed. The core is passed to the function and it can modify it's inner values

use winit::dpi::PhysicalSize;

use crate::ui::HystUi;

pub trait ShaderEvent: 'static {
    fn on_executed(&self, target: &mut HystUi);
}

impl ShaderEvent for PhysicalSize<u32> {
    fn on_executed(&self, target: &mut HystUi) {
        target.core_mut().resize(self.width, self.height);
        target.resize_roots(self.width as f32, self.height as f32);
    }
}
