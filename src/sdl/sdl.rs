extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::Color;
use self::sdl2::surface::Surface;

use std::path::Path;

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 240;
const SCREEN_SIZE: u32 = SCREEN_WIDTH * SCREEN_HEIGHT * 3; // In pixels times 3 for rgb


pub enum Input {
    Continue,
    Quit,
}

#[derive(Copy, Clone)]
pub enum ScreenSize {
    Default,
    Medium,
    Large,
}

impl ScreenSize {
    fn factor(self) -> u32 {
        match self {
            ScreenSize::Default => 1,
            ScreenSize::Medium => 2,
            ScreenSize::Large => 3,
        }
    }
}

pub struct SDLInterface<'a> {
    renderer: sdl2::render::Renderer<'a>,
    texture: sdl2::render::Texture,
    event_pump: sdl2::EventPump,
}

impl<'a> SDLInterface<'a> {
    pub fn new(scale: ScreenSize) -> SDLInterface<'a> {
        let sdl_context = sdl2::init().unwrap_or_else(
            |e| { panic!("Failed to initialize SDL: {}", e)}
        );

        let video_context = sdl_context.video().unwrap_or_else(
            |e| { panic!("Failed to initialize SDL Video: {}", e) }
        );
        let window = video_context.window(
            "GadgetNES",
            SCREEN_WIDTH * scale.factor(),
            SCREEN_HEIGHT * scale.factor(),
        )
            .position_centered()
            .build()
            .unwrap_or_else(
                |e| { panic!("Failed to initialize Window: {}", e) }
            );
        let renderer = window.renderer().build().unwrap();
        let texture = renderer.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGB24,
            SCREEN_WIDTH, SCREEN_HEIGHT
        ).unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
            
        SDLInterface {
            renderer: renderer,
            texture: texture,
            event_pump: event_pump,
        }
    }

    pub fn load_bmp<P: AsRef<Path>>(&mut self, path: P) {
        let bmp = Surface::load_bmp(path).unwrap();
        let bmp_texture = self.renderer.create_texture_from_surface(bmp).unwrap();
        self.renderer.clear();
        self.renderer.copy(&bmp_texture, None, None);
        self.renderer.present();
    }

    pub fn display_frame(&mut self, frame: &[u8; SCREEN_SIZE as usize]) {
        self.texture.update(None, frame, (SCREEN_WIDTH*3) as usize).unwrap();
        self.renderer.clear();
        self.renderer.copy(&self.texture, None, None);
        self.renderer.present();
    }
    pub fn check_input(&mut self) -> Input {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Input::Quit;
                },
                _ => {},
            }
        }
        return Input::Continue;
    }
}
