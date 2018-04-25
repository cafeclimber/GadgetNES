use super::sdl2::Sdl;
use super::sdl2::pixels::PixelFormatEnum::RGB24;
use super::sdl2::render::WindowCanvas;
use super::sdl2::render::{Texture, TextureCreator};
use super::sdl2::video::WindowContext;

pub const NES_WIDTH:  u32 = 256;
pub const NES_HEIGHT: u32 = 240;
pub const SCREEN_SIZE: usize = NES_WIDTH as usize * NES_HEIGHT as usize * 3; // R + G + B

pub struct Screen<'a> {
    pub canvas: WindowCanvas,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    texture: Texture<'a>
}

impl<'a> Screen<'a> {
    pub fn new(canvas: WindowCanvas, texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let texture = texture_creator.create_texture_streaming(
            RGB24,
            NES_WIDTH,
            NES_HEIGHT
        ).unwrap();

        Screen {
            canvas: canvas,
            texture_creator: texture_creator,
            texture: texture
        }
    }

    pub fn refresh(&mut self) {
        self.canvas.clear();
        let _ = self.canvas.copy(&self.texture, None, None);
        // TODO: Log any errors from copy
        self.canvas.present();
    }

    pub fn sdl(&self) -> Sdl {
        self.canvas.window().subsystem().sdl()
    }

    pub fn update(&mut self, ppu_screen: &[u8; SCREEN_SIZE]) {}
}
