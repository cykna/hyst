use hyst_math::vectors::Vec2f32;
use taffy::NodeId;

use crate::{
    core::RenderingCore,
    meshes::text::Text,
    ui::{HystElementKey, pulse::Pulse},
};

use super::HystElement;

pub struct HystText {
    key: HystElementKey,
    layout: NodeId,
    inner: Text,
    content: Pulse<String>,
    children: Vec<HystElementKey>,
}

pub struct TextCreationOption {
    pub(crate) key: HystElementKey,
    pub(crate) layout: NodeId,
    pub(crate) font_size: f32,
    pub(crate) line_height: f32,
    pub(crate) position: Vec2f32,
    pub(crate) content: Pulse<String>,
}

impl HystText {
    pub fn new(core: &mut RenderingCore, config: TextCreationOption) -> Self {
        let mut content = config.content;
        content.add_dependency(config.key);
        let text = Text::new(
            core,
            config.position,
            &content.read(),
            config.font_size,
            config.line_height,
        );
        Self {
            children: Vec::new(),
            key: config.key,
            layout: config.layout,

            inner: text,
            content,
        }
    }
    pub fn inner(&self) -> &Text {
        &self.inner
    }
}

impl HystElement for HystText {
    fn children(&self) -> &Vec<HystElementKey> {
        &self.children
    }
    fn update(&mut self, core: &mut RenderingCore) {
        core.set_text(&mut self.inner.buffer_mut(), &self.content.read());
    }
    fn id(&self) -> HystElementKey {
        self.key
    }
    fn layout(&self) -> taffy::NodeId {
        self.layout
    }
    fn resize(
        &mut self,
        core: &mut RenderingCore,
        screen_size: (f32, f32),
        layout: &taffy::Layout,
    ) {
    }
    fn render(&self, pass: &mut wgpu::RenderPass) {
        //Not implemented by the text itself
    }
}
