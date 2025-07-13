use glyphon::Buffer;
use hyst_math::vectors::Vec2f32;

use crate::core::RenderingCore;

pub struct Text {
    position: Vec2f32,
    buffer: Buffer,
}

impl Text {
    pub fn new(
        manager: &mut RenderingCore,
        position: Vec2f32,
        text: &str,
        font_size: f32,
        line_height: f32,
    ) -> Self {
        let buffer = manager.create_text_buffer(font_size, line_height, text);
        Self { position, buffer }
    }
    #[inline]
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    #[inline]
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    #[inline]
    ///The left corner where this text is located at.
    pub fn x(&self) -> f32 {
        self.position.x()
    }

    #[inline]
    ///The top corner where this text is located at.
    pub fn y(&self) -> f32 {
        self.position.y()
    }
}
