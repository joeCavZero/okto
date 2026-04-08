use raylib::prelude::*;

use crate::console::OktoConsole;

impl OktoConsole {
    pub fn read_color_from_color_memory(&self, address: u8) -> Color {
        let address_usz = address as usize;
        let color_r = self
            .color_memory
            .get(address_usz.saturating_mul(4))
            .cloned()
            .unwrap_or(0);
        let color_g = self
            .color_memory
            .get(address_usz.saturating_mul(4).saturating_add(1))
            .cloned()
            .unwrap_or(0);
        let color_b = self
            .color_memory
            .get(address_usz.saturating_mul(4).saturating_add(2))
            .cloned()
            .unwrap_or(0);
        let color_a = self
            .color_memory
            .get(address_usz.saturating_mul(4).saturating_add(3))
            .cloned()
            .unwrap_or(0);
    
        Color::new(
            color_r,
            color_g,
            color_b,
            color_a,
        )
    }
}
