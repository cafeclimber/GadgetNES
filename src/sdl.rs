extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::Color;
use self::sdl2::surface::Surface;

use std::path::Path;

const DEFAULT_SCREEN_WIDTH: u32 = 256;
static DEFAULT_SCREEN_HEIGHT: u32 = 240;

pub enum Input {
    Continue,
    Quit,
}

pub enum ScreenSize {
    Default,
    Medium,
    Large,
}

pub struct SDLInterface<'a> {
    sdl_context: sdl2::Sdl,
    renderer: sdl2::render::Renderer<'a>,
    event_pump: sdl2::EventPump,
}

impl<'a> SDLInterface<'a> {
    pub fn new() -> SDLInterface<'a> {
        let sdl_context = sdl2::init().unwrap_or_else(
            |e| { panic!("Failed to initialize SDL: {}", e)}
        );

        let video_context = sdl_context.video().unwrap_or_else(
            |e| { panic!("Failed to initialize SDL Video: {}", e) }
        );
        let window = video_context.window(
            "GadgetNES",
            DEFAULT_SCREEN_WIDTH,
            DEFAULT_SCREEN_HEIGHT,
        )
            .position_centered()
            .build()
            .unwrap_or_else(
                |e| { panic!("Failed to initialize Window: {}", e) }
            );
        let renderer = window.renderer().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
            
        SDLInterface {
            sdl_context: sdl_context,
            renderer: renderer,
            event_pump: event_pump,
        }
    }

    pub fn set_screen_size(&mut self, screen_size: ScreenSize) {
        let (width, height) = match screen_size {
            ScreenSize::Default => (256, 240),
            ScreenSize::Medium => (512, 480),
            ScreenSize::Large => (1024, 960),
        };
        {
            let mut window = self.renderer.window_mut().unwrap();
            window.set_size(width, height).unwrap();
        }
        self.renderer.set_logical_size(width, height).unwrap();
    }

    pub fn load_bmp<P: AsRef<Path>>(&mut self, path: P) {
        let bmp = Surface::load_bmp(path).unwrap();
        let bmp_texture = self.renderer.create_texture_from_surface(bmp).unwrap();
        self.renderer.clear();
        self.renderer.copy(&bmp_texture, None, None);
        self.renderer.present();
    }

    pub fn set_clear_color(&mut self, red: u8, green: u8, blue: u8) {
        let color = Color::RGB(red, green, blue);
        self.renderer.set_draw_color(color);
    }
    pub fn display(&mut self) {
        self.renderer.clear();
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
