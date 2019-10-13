use std;

use crate::i8080;

use sdl2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub trait IO {
    fn input(&self, port: u8) -> u8;
    fn output(&mut self, port: u8, value: u8);
}

pub struct SpaceInvaders {
    pub cpu: i8080::cpu::CPU,
    io: crate::invaders::SpaceInvadersIO,

    pub instructions: u64,
    pub cycles: u64,
    pub frames: u64,

    sdl_context: sdl2::Sdl,
    screen: Screen,
    event_pump: sdl2::EventPump,
}

impl SpaceInvaders {
    const CYCLES_PER_FRAME: u64 = 4_000_000 / 60;
    pub const SCREEN_WIDTH: usize = 224;
    pub const SCREEN_HEIGHT: usize = 256;

    pub fn new(rom: &[u8]) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let screen = Screen::new(&sdl_context).unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            cpu: i8080::cpu::CPU::new(),
            io:  SpaceInvadersIO::new(),
            instructions: 0,
            cycles: 0,
            frames: 0,
            sdl_context: sdl_context,
            screen: screen,
            event_pump: event_pump,
        }
    }

    pub fn step(&mut self) {
        self.half_step(true);
        self.half_step(false);

        self.frames += 1;

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    pub fn half_step(&mut self, top_half: bool) {
        let mut cycles_spent = 0;
        while cycles_spent < Self::CYCLES_PER_FRAME / 2 {

            let cycles1 = self.cpu.tick(&mut self.io);
            cycles_spent += cycles1;
            self.instructions += 1;
            self.cycles += cycles1;
        }

        self.cpu.interrupt(if top_half { 1 } else { 2 });

        self.draw_from_memory();
    }

    pub fn draw_from_memory(&mut self) {
        let framebuffer = self.cpu.memory.view(0x2400, 0x3FFF);

        self.screen.clear();

        for y in 0..224 {
            let line = &framebuffer[(32 * y)..(32 * y + 32)];
            for (x, px) in line.iter().enumerate() {
                for b in 0..8 {
                    if px & (1 << b) != 0 {
                        self.screen.draw(y as i16, 256 - (8 * x + b) as i16, 255).unwrap();
                    }
                }
            }
        }
        self.screen.canvas.present();
    }
}

pub struct SpaceInvadersIO {
    shift0: u8,
    shift1: u8,
    shift_amount: u8,
    port0: u8,
    port1: u8,
    port2: u8,
}

impl SpaceInvadersIO {
    pub fn new() -> Self {
        Self {
            shift0: 0,
            shift1: 0,
            shift_amount: 0,
            port0: 0b0111_0000,
            port1: 0b0001_0000,
            port2: 0b0000_0000,
        }
    }

    fn update_input(&mut self) {

    }

    fn set_key(port: &mut u8, bit: u8, on: bool) {
        if on {
            *port |= 1 << bit
        } else {
            *port &= !(1 << bit)
        }
    }
}

impl IO for SpaceInvadersIO {
    fn input(&self, port: u8) -> u8 {
        match port {
            1 => self.port1,
            2 => self.port2,
            3 => {
                let reg = u16::from(self.shift1) << 8 | u16::from(self.shift0);
                ((reg >> (8 - self.shift_amount)) as u8)
            },
            _ => panic!("Cannot read port: {}", port),
        }
    }

    fn output(&mut self, port: u8, value: u8) {
        match port {
            2 => self.shift_amount = value & 0b111,
            4 => {
                self.shift0 = self.shift1;
                self.shift1 = value;
            }
            3 | 5 | 6 => {}
            _ => panic!("Cannot write to port: {}", port),
        }
    }
}

pub struct Screen {
    pub video: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::WindowCanvas,
    scale_factor: i16,
}

impl Screen {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Screen, String> {
        let video = sdl_context.video()?;
        let window = video
            .window("Space Invaders", 256 * 2, 256 * 2)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0,0,0));
        canvas.clear();
        canvas.present();

        Ok(Screen {
            video,
            canvas,
            scale_factor: 2,
        })
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn draw(&mut self, x: i16, y: i16, colour: u8) -> Result<(), String> {
        let color = pixels::Color::RGB(colour, colour, colour);
        self.canvas.box_(
            x * self.scale_factor,
            y * self.scale_factor,
            x * self.scale_factor + self.scale_factor - 1,
            y * self.scale_factor + self.scale_factor - 1,
            color,
        )?;

        Ok(())
    }
}