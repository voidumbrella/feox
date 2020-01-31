use sdl2::pixels::PixelFormatEnum;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::gfx::framerate::FPSManager;

pub struct Gui {
    pub context: sdl2::Sdl,
    canvas: Canvas<Window>,
    fps_man: FPSManager,
}

impl Gui {
    pub fn new(fps: u32) -> Result<Self, String> {
        let context = sdl2::init()
            .map_err(|e| e.to_string())?;
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem.window("feox", 320, 288)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build()
            .map_err(|e| e.to_string())?;
        
        let mut fps_man = FPSManager::new();
        fps_man.set_framerate(fps).map_err(|e| e.to_string())?;

        Ok(Self {
            context,
            canvas,
            fps_man,
        })
    }

    pub fn update_screen(&mut self, bytes: &[u8]) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 160, 144)
            .map_err(|e| e.to_string())?;
        texture.update(None, &bytes, 3 * 160)
            .map_err(|e| e.to_string())?;
        self.canvas.copy(&texture, None, None)?;
        self.canvas.present();

        Ok(())
    }

    pub fn delay(&mut self) {
        self.fps_man.delay();
    }

    pub fn set_framerate(&mut self, fps: u32) -> Result<(), String> {
        self.fps_man.set_framerate(fps)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
