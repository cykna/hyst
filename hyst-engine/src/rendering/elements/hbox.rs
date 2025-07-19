use crate::{core::RenderingCore, ui::HystElementKey};
use taffy::NodeId;

use super::HystElement;

pub struct HystBoxCreationOption {
    pub index: u64,
    pub parent: Option<HystElementKey>,
    pub style: NodeId,
    pub key: HystElementKey,
}

#[derive(Debug)]
pub struct HystBox {
    ///The parent key on the Ui tree
    parent: Option<HystElementKey>,
    //The children keys on the Ui tree
    children: Vec<HystElementKey>,
    //The key on the Ui tree
    key: HystElementKey,
    style: NodeId,
    ///The instance index on the batch renderer
    index: u64,
}

impl HystBox {
    pub fn new(config: HystBoxCreationOption) -> Self {
        Self {
            index: config.index,
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
}

impl HystElement for HystBox {
    fn instance_index(&self) -> u64 {
        self.index
    }
    fn id(&self) -> HystElementKey {
        self.key
    }
    fn layout(&self) -> NodeId {
        self.style
    }
    fn children(&self) -> &Vec<HystElementKey> {
        &self.children
    }
    fn update(&mut self, core: &mut RenderingCore) {}
    fn render(&self, pass: &mut wgpu::RenderPass) {}
}
