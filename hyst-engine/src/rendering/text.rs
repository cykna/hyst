use glyphon::{
    Attrs, Buffer, Cache, Color, FontSystem, Metrics, Resolution, SwashCache, TextAtlas,
    TextBounds, TextRenderer, Viewport,
};
use hyst_math::vectors::Vec2f32;
use wgpu::{Device, Queue, RenderPass, TextureFormat};

use super::meshes::text::Text;

///This struct is used for managning and rendering texts on the screen.
pub struct TextManager {
    font_sys: FontSystem,
    swash_cache: SwashCache,
    cache: Cache,
    viewport: Viewport,
    atlas: TextAtlas,
    renderer: TextRenderer,
    size: (f32, f32),
}

impl TextManager {
    pub fn new(device: &Device, queue: &Queue, texture: TextureFormat, size: (f32, f32)) -> Self {
        let cache = Cache::new(device);
        let mut atlas = TextAtlas::new(device, queue, &cache, texture);
        Self {
            size,
            font_sys: FontSystem::new(),
            swash_cache: SwashCache::new(),
            viewport: Viewport::new(device, &cache),
            renderer: TextRenderer::new(
                &mut atlas,
                device,
                wgpu::MultisampleState::default(),
                None,
            ),
            atlas,
            cache,
        }
    }

    ///Creates a buffer with the given `font_size`, `line_height` and `text`
    pub fn create_text_buffer(&mut self, font_size: f32, line_height: f32, text: &str) -> Buffer {
        let mut buffer = Buffer::new(&mut self.font_sys, Metrics::new(font_size, line_height));
        buffer.set_text(
            &mut self.font_sys,
            text,
            &Attrs::new().family(glyphon::Family::Serif),
            glyphon::Shaping::Basic,
        );
        println!("vec2({:?},{:?})", self.size.0, self.size.1);
        buffer.set_size(&mut self.font_sys, Some(self.size.0), Some(self.size.1));
        buffer.shape_until_scroll(&mut self.font_sys, true);
        buffer
    }

    ///Sets the content of the given `buffer` to be the given `text`
    pub fn set_text(&mut self, buffer: &mut Buffer, text: &str) {
        buffer.set_text(
            &mut self.font_sys,
            text,
            &Attrs::new().family(glyphon::Family::Serif),
            glyphon::Shaping::Basic,
        );
        buffer.shape_until_scroll(&mut self.font_sys, true);
    }

    #[inline]
    ///Sets the given `metrics` for the given `buffer`
    pub fn set_metrics(&mut self, buffer: &mut Buffer, metrics: Metrics) {
        buffer.set_metrics(&mut self.font_sys, metrics);
    }
    #[inline]
    ///Prepare the given texts for rendering
    pub fn prepare(&mut self, device: &Device, queue: &Queue, texts: Vec<(Buffer, Vec2f32)>) {
        self.renderer
            .prepare(
                device,
                queue,
                &mut self.font_sys,
                &mut self.atlas,
                &mut self.viewport,
                texts.iter().map(|(buffer, v2)| glyphon::TextArea {
                    buffer,
                    left: 0.0,
                    top: 0.0,
                    scale: 1.0,
                    bounds: {
                        let x = v2.x() as i32;
                        let y = v2.y() as i32;
                        let (Some(w), Some(h)) = buffer.size() else {
                            panic!("Buffer should have size. Error found");
                        };
                        TextBounds {
                            left: 0,
                            top: 0,
                            right: 1000 as i32,
                            bottom: 1000 as i32,
                        }
                    },
                    default_color: Color::rgba(255, 0, 255, 255),
                    custom_glyphs: &[],
                }),
                &mut self.swash_cache,
            )
            .unwrap();
    }
    ///Resizes this text renderer viewport
    pub fn resize(&mut self, queue: &Queue, width: u32, height: u32) {
        self.viewport.update(queue, Resolution { width, height });
    }

    #[inline]
    ///Draws the prepared texts
    pub fn draw_texts(&self, rpass: &mut RenderPass) {
        self.renderer
            .render(&self.atlas, &self.viewport, rpass)
            .unwrap();
    }
}
