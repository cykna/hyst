use crate::{
    AbstractBuffer,
    background::Background,
    core::RenderingCore,
    meshes::{Mesh, SizeMethod, container::Container},
    ui::HystElementKey,
};
use hyst_math::Rect;

pub struct HystBoxCreationOption {
    pub background: Background,
    pub rect: Rect,
    pub size_method: SizeMethod,
    pub parent: Option<HystElementKey>,
    pub key: HystElementKey,
}

pub struct HystBox {
    container: Container,
    size_method: SizeMethod,
    parent: Option<HystElementKey>,
    children: Vec<HystElementKey>,
    key: HystElementKey,
}

impl HystBox {
    pub fn new(core: &mut RenderingCore, config: HystBoxCreationOption) -> Self {
        let container = Container::new(core, config.background, config.rect);
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
