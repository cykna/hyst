use crate::{
    batch::BatchRenderer,
    core::RenderingCore,
    meshes::{
        container::{Container, ContainerInput, ContainerInstance},
        image::{Image, ImageInput},
    },
};

pub struct Renderer {
    box_renderer: BatchRenderer<Container>,
    //image_renderer: BatchRenderer<Image>,
}

impl Renderer {
    pub fn new(core: &mut RenderingCore) -> Self {
        let window_size = {
            let size = core.size();
            [size.0 as f32, size.1 as f32]
        };
        Self {
            box_renderer: BatchRenderer::new(
                core,
                &[
                    ContainerInput::new(-1.0, 1.0),
                    ContainerInput::new(1.0, 1.0),
                    ContainerInput::new(-1.0, -1.0),
                    ContainerInput::new(1.0, -1.0),
                ],
                include_str!("../../../../shaders/container.wgsl"),
                "container",
                &[0, 1, 2, 2, 1, 3],
                vec![],
                window_size,
            ),
            // image_renderer: BatchRenderer::new(
            //     core,
            //     &[
            //         ImageInput::new(-1.0, 1.0, 0.0, 0.0),
            //         ImageInput::new(1.0, 1.0, 1.0, 0.0),
            //         ImageInput::new(-1.0, -1.0, 0.0, 1.0),
            //         ImageInput::new(1.0, -1.0, 1.0, 1.0),
            //     ],
            //     include_str!("../../../../shaders/image.wgsl"),
            //     "img",
            //     &[0, 1, 2, 2, 1, 3],
            //     vec![],
            //     window_size,
            // ),
        }
    }

    ///Sets the window size of all renderers to be the given one
    /// # Arguments
    /// * `width` The width of the window
    /// * `height` The height of the window
    ///
    #[inline]
    pub fn set_size(&mut self, core: &RenderingCore, width: f32, height: f32) {
        self.box_renderer.set_window_size(core, [width, height]);
    }

    ///Inserts a new box at the box renderer
    #[inline]
    pub fn insert_box(&mut self, instance: ContainerInstance) -> u64 {
        self.box_renderer.push(instance)
    }

    pub fn box_renderer(&self) -> &BatchRenderer<Container> {
        &self.box_renderer
    }

    pub fn box_renderer_mut(&mut self) -> &mut BatchRenderer<Container> {
        &mut self.box_renderer
    }

    ///Flushes all the modifications made on every internal renderers
    pub fn flush_modifications(&self, core: &RenderingCore) {
        self.box_renderer().flush(core);
        // self.image_renderer().flush(core);
    }

    // pub fn image_renderer(&self) -> &BatchRenderer<Image> {
    //     &self.image_renderer
    // }

    // pub fn image_renderer_mut(&mut self) -> &mut BatchRenderer<Image> {
    //     &mut self.image_renderer
    // }
}
