
use raylib::prelude::*;

const MEMORY_SIZE_8: usize = 256;
const MEMORY_SIZE_16: usize = 65536;

const OKTO_CONSOLE_SCREEN_WIDTH: i32 = 240;
const OKTO_CONSOLE_SCREEN_HEIGHT: i32 = 240;

pub struct OktoConsole {
    pub raylib: RaylibHandle,
    pub raylib_tread: RaylibThread,
    pub canvas: RenderTexture2D,

    pub color_memory: [u8; MEMORY_SIZE_8],
    pub palette_memory: [u8; MEMORY_SIZE_8],
    pub sprite_memory: [u8; MEMORY_SIZE_16],
    pub audio_memory: [u8; MEMORY_SIZE_16],
}

impl OktoConsole {

    pub fn from(color: Vec<u8>, palette: Vec<u8>, sprite: Vec<u8>, audio: Vec<u8>) -> Self {
        // color
        let mut color_memory = [0; MEMORY_SIZE_8];

        for i in 0..color_memory.len() {
            color_memory[i] = rand::random::<u8>();
        }

        for i in 0..color.len() {
            match color_memory.get_mut(i) {
                Some(b) => *b = color[i],
                None => break,
            }
        }
        
        // palette
        let mut palette_memory = [0; MEMORY_SIZE_8];

        for i in 0..palette_memory.len() {
            palette_memory[i] = rand::random::<u8>();
        }

        for i in 0..palette.len() {
            match palette_memory.get_mut(i) {
                Some(b) => *b = palette[i],
                None => break,
            }
        }

        // sprite
        let mut sprite_memory = [0; MEMORY_SIZE_16];

        for i in 0..sprite_memory.len() {
            sprite_memory[i] = rand::random::<u8>();
        }

        for i in 0..sprite.len() {
            match sprite_memory.get_mut(i) {
                Some(b) => *b = sprite[i],
                None => break,
            }
        }

        

        // audio

        let mut audio_memory = [0; MEMORY_SIZE_16];

        for i in 0..audio_memory.len() {
            audio_memory[i] = rand::random::<u8>();
        }

        for i in 0..audio.len() {
            match audio_memory.get_mut(i) {
                Some(b) => *b = audio[i],
                None => break,
            }
        }

        // RAYLIB

        let (mut rl, thread) = raylib::init()
            .size(OKTO_CONSOLE_SCREEN_WIDTH, OKTO_CONSOLE_SCREEN_HEIGHT)
            .title("okto")
            .log_level(TraceLogLevel::LOG_NONE)
            .resizable()
            .build();

        let canvas = rl
            .load_render_texture(&thread, OKTO_CONSOLE_SCREEN_WIDTH as u32, OKTO_CONSOLE_SCREEN_HEIGHT as u32)
            .expect("Failed to create canvas");

        Self {
            color_memory,
            palette_memory,
            sprite_memory,
            audio_memory,

            raylib: rl,
            raylib_tread: thread,
            canvas,
        }
    }

    pub fn present_canvas(&mut self) {
        let window_size = self.get_window_size();
        let canvas_size = Vector2::new(OKTO_CONSOLE_SCREEN_WIDTH as f32, OKTO_CONSOLE_SCREEN_HEIGHT as f32);

        let mut d = self.raylib.begin_drawing(&self.raylib_tread);
        d.clear_background(Color::BLACK);

        render_canvas_on_draw_handle(
            &mut d,
            window_size,
            &self.canvas,
            canvas_size,
        );
    }

    pub fn clear_canvas(&mut self) {
        let mut t = self.raylib.begin_texture_mode(&self.raylib_tread, &mut self.canvas);
        t.clear_background(Color::new(25,25,25,255));
    }

    pub fn get_window_size(&self) -> Vector2 {
        Vector2::new(
            self.raylib.get_screen_width() as f32,
            self.raylib.get_screen_height() as f32,
        )
    }

    pub fn window_should_close(&self) -> bool {
        return self.raylib.window_should_close()
    }
}



pub fn render_canvas_on_draw_handle(draw_handle: &mut RaylibDrawHandle<'_>, window_size: Vector2, canvas: &RenderTexture2D, canvas_size: Vector2) {
    
    let delta_x = window_size.x / canvas_size.x;
    let delta_y = window_size.y / canvas_size.y;

    let scale = if delta_x < delta_y { delta_x } else { delta_y };

    let scaled_width = canvas_size.x as f32 * scale;
    let scaled_height = canvas_size.y as f32 * scale;

    let diff_x = window_size.x as f32 - scaled_width;
    let diff_y = window_size.y as f32 - scaled_height;

    draw_handle.draw_texture_pro(
        canvas.texture(),
        Rectangle::new(
            0.0,
            0.0,
            canvas_size.x as f32,
            -(canvas_size.y as f32),
        ),
        Rectangle::new(diff_x / 2.0, diff_y / 2.0, scaled_width, scaled_height),
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}