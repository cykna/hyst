use crate::{
    core::RenderingCore,
    mesh::{Mesh, SizeMethod},
    meshes::container::{AbstractBuffer, Container},
    rectangle::Rect,
    ui::{HystElementKey, HystElementOptions},
};

pub struct HystBox {
    container: Container,
    size_method: SizeMethod,
    parent: Option<HystElementKey>,
    children: Vec<HystElementKey>,
    key: HystElementKey,
}

impl HystBox {
    pub fn new(core: &mut RenderingCore, config: HystElementOptions) -> Self {
        let container = Container::new(core, config.background, config.initial_rect);
        Self {
            container,
            size_method: config.size_method,
            parent: config.parent,
            children: Vec::new(),
            key: config.key,
        }
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

    pub fn size_method(&self) -> SizeMethod {
        self.size_method
    }

    pub fn rect(&mut self) -> &mut AbstractBuffer<Rect> {
        self.container.area_buffer()
    }

    pub fn screen_size(&mut self) -> &mut AbstractBuffer<[f32; 2]> {
        self.container.screen_size()
    }
}
