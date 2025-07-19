use hyst_math::vectors::{Vec2f32, Vec4f32};
use taffy::NodeId;

use crate::{
    batch::BatchSubmitter,
    core::RenderingCore,
    meshes::text::Text,
    ui::{HystElementKey, pulse::Pulse},
};

use super::HystElement;

#[derive(Debug)]
pub struct HystText {
    key: HystElementKey,
    layout: NodeId,
    inner: Text,
    content: Pulse<String>,
    color: Pulse<Vec4f32>,
    children: Vec<HystElementKey>,
}

pub struct TextCreationOption {
    pub(crate) key: HystElementKey,
    pub(crate) layout: NodeId,
    pub(crate) font_size: f32,
    pub(crate) line_height: f32,
    pub(crate) position: Vec2f32,
    pub(crate) content: Pulse<String>,
    pub(crate) color: Pulse<Vec4f32>,
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
            color: config.color,
            inner: text,
            content,
        }
    }
    pub fn inner(&self) -> &Text {
        &self.inner
    }
    pub fn color(&self) -> &Pulse<Vec4f32> {
        &self.color
    }
    pub fn color_mut(&mut self) -> &mut Pulse<Vec4f32> {
        &mut self.color
    }
}

impl HystElement for HystText {
    fn instance_index(&self) -> u64 {
        0
    }
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
    fn render(&self, _: &mut wgpu::RenderPass) {
        //Not implemented by the text itself
    }
}
