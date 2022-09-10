use crate::core::chip8::Chip8;
use crate::core::rom::Rom;
use sdl2::render;

pub struct Emulator {
    chip8: Chip8,
    window: render::WindowCanvas,
}

impl Emulator {
    pub fn new(rom: Rom) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let scale = 1; //TODO: change not hard code here pls...

        let window = video_subsystem
            .window("YAARCH8", 64 * scale, 32 * scale)
            .position_centered()
            .build()
            .unwrap();

        // canvas is our screen where we draw sprite
        let canvas = window.into_canvas().build().unwrap();

        Self {
            chip8: Chip8::new(),
            window: canvas,
        }
    }
}
