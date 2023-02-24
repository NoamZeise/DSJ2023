use std::path::Path;

use nze_game_sdl::{
    DrawingArea,
    Camera,
    Render,
    input::Controls,
    input::keyboard::Key,
    geometry::{Rect, Vec2},
    Error,
    map::Map,
};

pub fn main() -> Result<(), Error> {
    let (mut cam, drawing_area, context) = DrawingArea::new(
        "DSJ 2023",
        Rect::new(0.0, 0.0, 240.0, 180.0),
        Vec2::new(480.0, 360.0)
    )?;
    let mut render = Render::new(drawing_area, &context)?;
    let mut controls = Controls::new(&context)?;
    let mut game = Game::new(&mut render)?;

    while !controls.should_close {
        controls.update(&cam);
        game.update(&mut controls);
        render.start_draw();
        game.draw(&mut cam);
        render.end_draw(&mut cam)?;
    }
    
    Ok(())
}

const SPEED: f64 = 200.0;
struct Game {
    map: Map,
    mov: Vec2,
}

impl Game {
    pub fn new(render: &mut Render) -> Result<Game, Error> {
        Ok(Game {
            map: Map::new(
                Path::new("resources/map/tiled-ex.tmx"),
                &mut render.texture_manager,
                Path::new("resources/fonts/"),
                &mut render.font_manager
            )?,
            mov: Vec2::zero(),
        })
    }

    pub fn update(&mut self, controls: &mut Controls) {
        if controls.kb.press(Key::Escape) {
            controls.should_close = true;
        }
        self.mov = Vec2::zero();
        if controls.kb.down(Key::W) {
            self.mov.y -= SPEED * controls.frame_elapsed;
        }
        if controls.kb.down(Key::A) {
            self.mov.x -= SPEED * controls.frame_elapsed;
        }
        if controls.kb.down(Key::S) {
            self.mov.y += SPEED * controls.frame_elapsed;
        }
        if controls.kb.down(Key::D) {
            self.mov.x += SPEED * controls.frame_elapsed;
        }
    }
    pub fn draw(&mut self, cam: &mut Camera) {
        cam.set_offset(cam.get_offset() + self.mov);
        self.map.draw(cam);
    }
}
