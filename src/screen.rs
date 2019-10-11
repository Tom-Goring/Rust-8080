use sdl2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;

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