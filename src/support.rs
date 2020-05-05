#![allow(dead_code)]


use std;
use glium;

pub struct GliumDisplayWinitWrapper(pub glium::Display);

impl conrod_winit::WinitWindow for GliumDisplayWinitWrapper {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        self.0.gl_window().get_inner_size().map(Into::into)
    }

    fn hidpi_factor(&self) -> f32 {
        self.0.gl_window().get_hidpi_factor() as f32
    }
}

conrod_winit::conversion_fns!();