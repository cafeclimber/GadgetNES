// Big thanks to the work done by pcwalton and his SprocketNES project here.
// I know very little about SDL and graphics and his was a huge help.

//! Provides an interface to SDL.

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::pixels::PixelFormatEnum;

pub const SCREEN_WIDTH:  usize = 256;
pub const SCREEN_HEIGHT: usize = 240;
pub const SCREEN_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT * 3); 

pub struct Graphics<'a> {
    renderer: Renderer<'a>,
    texture: Texture,
}

impl<'a> Graphics<'a> {
    pub fn new(sdl_context: &Sdl) -> Graphics<'a> {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("GadgetNES",
                                            SCREEN_WIDTH as u32,
                                            SCREEN_HEIGHT as u32)
            .position_centered()
            .build().unwrap();

        let mut renderer = window.renderer().build().unwrap();

        let mut texture = renderer.create_texture_streaming(
            PixelFormatEnum::RGB24,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32).unwrap();

        Graphics {
            renderer: renderer,
            texture: texture,
        }
    }

    pub fn display_frame(&mut self, ppu_frame: &mut[u8; SCREEN_SIZE]) {
        self.blit(ppu_frame);
        self.renderer.clear();
        self.renderer.copy(&self.texture, None, None);
        self.renderer.present();
    }

    /// Updates the texture with one frame of pixel data
    fn blit(&mut self, ppu_frame: &mut[u8; SCREEN_SIZE]) {
        self.texture.update(None, ppu_frame, SCREEN_WIDTH * 3).unwrap();
    }
}
