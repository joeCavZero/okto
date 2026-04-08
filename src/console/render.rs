use raylib::prelude::*;

use crate::console::OktoConsole;

impl OktoConsole {
    fn get_internal_x(x: u8) -> f32 {
        x as f32 - 8.0
    }
    fn get_internal_y(y: u8) -> f32 {
        y as f32 - 8.0
    }

    fn get_internal_vector2(x: u8, y: u8) -> Vector2 {
        Vector2::new(
            Self::get_internal_x(x), 
            Self::get_internal_y(y),
        )
    }

    fn get_vector2(w: u8, h: u8) -> Vector2 {
        Vector2::new(w as f32, h as f32)
    }

    pub fn render_pixel(
        &mut self,
        x: u8,
        y: u8,
        color: Color,
    ) {

        let pos = Self::get_internal_vector2(x, y);
        let mut t = self.raylib.begin_texture_mode(&self.raylib_tread, &mut self.canvas);
        t.draw_pixel_v(pos, color);

    }

    pub fn render_line(
        &mut self,
        x1: u8,
        y1: u8,
        x2: u8,
        y2: u8,
        thickness: u8,
        color: Color,
    ) {

        let start = Self::get_internal_vector2(x1, y1);
        let end = Self::get_internal_vector2(x2, y2);
        let mut t = self.raylib.begin_texture_mode(&self.raylib_tread, &mut self.canvas);
        t.draw_line_ex(start, end, thickness as f32, color);

    }

    pub fn render_circle(
        &mut self,
        x: u8,
        y: u8,
        radius: u8,
        color: Color,
    ) {

        let pos = Self::get_internal_vector2(x, y);
        let mut t = self.raylib.begin_texture_mode(&self.raylib_tread, &mut self.canvas);
        t.draw_circle_v(pos, radius as f32, color);

    }

    pub fn render_rect(
        &mut self,
        x: u8,
        y: u8,
        w: u8,
        h: u8,
        color: Color,
    ) {
        let pos = Self::get_internal_vector2(x, y);
        let size = Self::get_vector2(w, h);

        let mut t = self.raylib.begin_texture_mode(&self.raylib_tread, &mut self.canvas);
        t.draw_rectangle_v(pos, size, color);
    }

    pub fn render_triangle(
        &mut self,
        x1: u8,
        y1: u8,
        x2: u8,
        y2: u8,
        x3: u8,
        y3: u8,
        color: Color,
    ) {

        let pos1 = Self::get_internal_vector2(x1, y1);
        let pos2 = Self::get_internal_vector2(x2, y2);
        let pos3 = Self::get_internal_vector2(x3, y3);
        let mut t = self.raylib.begin_texture_mode(&self.raylib_tread, &mut self.canvas);
        t.draw_triangle(pos1, pos2, pos3, color);
    }
    
}

