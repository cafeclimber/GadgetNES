extern crate sdl2;

use self::sdl2::EventPump;
use self::sdl2::render::Renderer;
use self::sdl2::video::Window;
use self::sdl2::Sdl;

const SCREEN_WIDTH:  u32 = 256;
const SCREEN_HEIGHT: u32 = 240;

pub struct GraphicsInterface<'a> {
    renderer: Renderer<'a>,

}

impl<'a> GraphicsInterface<'a> {
    pub fn new() -> (GraphicsInterface<'a>, Sdl) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("GadgetNES",
                                            SCREEN_WIDTH,
                                            SCREEN_HEIGHT)
            .position_centered()
            .build().unwrap();

        let mut renderer = window.renderer().build().unwrap();

        (GraphicsInterface {
            renderer: renderer,
        }, sdl_context)
    }
}
