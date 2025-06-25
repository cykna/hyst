use crate::{
    AbstractBuffer,
    background::Background,
    core::RenderingCore,
    meshes::{Mesh, container::Container},
    ui::HystElementKey,
};
use hyst_math::Rect;
use taffy::NodeId;

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
    pub fn new(core: &mut RenderingCore, config: HystBoxCreationOption) -> Self {
        let container = Container::new(core, config.background, config.rect);
        Self {
            container,
            parent: config.parent,
            children: Vec::new(),
            key: config.key,
            style: config.style,
        }
    }

    pub fn style(&self) -> NodeId {
        self.style
    }

    pub fn children(&self) -> &[HystElementKey] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<HystElementKey> {
        &mut self.children
    }

    pub fn key(&self) -> HystElementKey {
        self.key
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

    pub fn rect(&mut self) -> &mut AbstractBuffer<Rect> {
        self.container.area_buffer()
    }

    pub fn screen_size(&mut self) -> &mut AbstractBuffer<[f32; 2]> {
        self.container.screen_size()
    }
}
