use sdl2::pixels::Color;

pub struct Ground {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Ground {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(sdl2::rect::Rect::new(
            self.x as i32,
            self.y as i32,
            self.width as u32,
            self.height as u32,
        ))
    }
}
