use crate::{
    batch::BatchRenderer,
    core::RenderingCore,
    meshes::{
        container::{Container, ContainerInput},
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

    pub fn box_renderer(&self) -> &BatchRenderer<Container> {
        &self.box_renderer
    }

    pub fn box_renderer_mut(&mut self) -> &mut BatchRenderer<Container> {
        &mut self.box_renderer
    }

    // pub fn image_renderer(&self) -> &BatchRenderer<Image> {
    //     &self.image_renderer
    // }

    // pub fn image_renderer_mut(&mut self) -> &mut BatchRenderer<Image> {
    //     &mut self.image_renderer
    // }
}
