use crate::{
    AbstractBuffer,
    background::Background,
    batch::{BatchRenderer, BatchSubmitter},
    core::RenderingCore,
    meshes::{Mesh, container::Container},
    ui::HystElementKey,
};
use hyst_math::Rect;
use taffy::NodeId;

use super::HystElement;

pub struct HystBoxCreationOption {
    pub background: Background,
    pub rect: Rect,
    pub parent: Option<HystElementKey>,
    pub style: NodeId,
    pub key: HystElementKey,
}

#[derive(Debug)]
pub struct HystBox {
    container: Container,
    parent: Option<HystElementKey>,
    children: Vec<HystElementKey>,
    style: NodeId,
    key: HystElementKey,
}

impl HystBox {
    pub fn new(config: HystBoxCreationOption) -> Self {
        let container = Container::new(config.background, config.rect, 0);
        Self {
            container,
            parent: config.parent,
            children: Vec::new(),
            key: config.key,
            style: config.style,
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<HystElementKey> {
        &mut self.children
    }

    pub fn parent(&self) -> Option<&HystElementKey> {
        self.parent.as_ref()
    }

    pub fn container(&self) -> &Container {
        &self.container
    }

    pub fn container_mut(&mut self) -> &mut Container {
        &mut self.container
    }
}

impl HystElement for HystBox {
    fn id(&self) -> HystElementKey {
        self.key
    }
    fn layout(&self) -> NodeId {
        self.style
    }
    fn resize(
        &mut self,
        core: &RenderingCore,
        renderer: &mut dyn BatchSubmitter,
        layout: &taffy::Layout,
    ) {
        self.container.resize(core, renderer, layout);
    }
    fn children(&self) -> &Vec<HystElementKey> {
        &self.children
    }
    fn update(&mut self, core: &mut RenderingCore) {}
    fn render(&self, pass: &mut wgpu::RenderPass) {}
}
