pub mod error;
use ahash::RandomState;
use error::LayoutError;
use smol_str::SmolStr;
use taffy::{Dimension, Layout, NodeId, Style, TaffyError, TaffyTree};
pub type StyleMap = hashbrown::HashMap<SmolStr, Style, RandomState>;

///Struct responsible for managing the layouts of the Hyst engine. Not that this only manages
///how elements will be positioned
pub struct HystLayout {
    taffy: TaffyTree<()>,
    styles: StyleMap,
    root: NodeId,
}

impl HystLayout {
    pub fn new() -> Self {
        let mut taffy = TaffyTree::new();
        let root = taffy
            .new_leaf(taffy::Style {
                size: taffy::Size {
                    width: Dimension::percent(1.0),
                    height: Dimension::percent(1.0),
                },
                ..Default::default()
            })
            .unwrap();
        Self {
            taffy,
            root,
            styles: StyleMap::with_hasher(RandomState::new()),
        }
    }
    ///Gets the style at the specified index which is child of `parent`
    #[inline]
    pub fn get_style(&self, index: u32, parent: Option<NodeId>) -> Result<NodeId, LayoutError> {
        self.taffy
            .child_at_index(parent.unwrap_or(self.root), index as usize)
            .map_err(LayoutError::Taffy)
    }

    pub fn layout_of(&self, node: NodeId) -> Result<&Layout, LayoutError> {
        self.taffy.layout(node).map_err(LayoutError::Taffy)
    }

    pub fn root(&self) -> NodeId {
        self.root
    }

    ///Create a style with the given name. Note: this is used as a base, it will not be recalculated when things change.
    ///So 3 elements can have the same style but yet be positioned and have different rect sizes.
    pub fn create_style(&mut self, name: SmolStr, style: Style) {
        self.styles.insert(name, style);
    }

    ///Returns a list of ids of the created styles. Errors if some style name doesn't exist.
    /// # Arguments
    /// * `parent` The parent node that this style is children of. If None, is the root id
    pub fn create_element_style(
        &mut self,
        parent: Option<NodeId>,
        style: SmolStr,
    ) -> Result<NodeId, LayoutError> {
        let parent = parent.unwrap_or(self.root);

        if let Some(style) = self.styles.get(&style) {
            let id = self
                .taffy
                .new_leaf(style.clone())
                .map_err(LayoutError::Taffy)?;
            self.taffy
                .add_child(parent, id)
                .map_err(LayoutError::Taffy)?;
            Ok(id)
        } else {
            Err(LayoutError::InvalidStyleName(style.to_string()))
        }
    }

    ///Recalculates the tree based on the given `width` and `height`.
    pub fn recalculate(&mut self, width: f32, height: f32) -> Result<(), TaffyError> {
        self.taffy.compute_layout(
            self.root,
            taffy::Size {
                width: taffy::AvailableSpace::Definite(width),
                height: taffy::AvailableSpace::Definite(height),
            },
        )
    }
}
