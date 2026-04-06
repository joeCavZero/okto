use raylib::prelude::*;

use crate::console::OktoConsole;

impl OktoConsole {
    pub fn render_line(
        &mut self,
        x1: u8,
        y1: u8,
        x2: u8,
        y2: u8,
        thickness: u8,
        color: Color,
    ) {

        let start = Vector2::new(x1 as f32, y1 as f32);
        let end = Vector2::new(x2 as f32, y2 as f32);
        let mut t = self.raylib.begin_texture_mode(&self.raylib_tread, &mut self.canvas);
        t.draw_line_ex(start, end, thickness as f32, color);

    }
}