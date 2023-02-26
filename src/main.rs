use nze_game_sdl::{
    DrawingArea,
    Render,
    input::Controls,
    geometry::{Rect, Vec2},
    Error,
};

use deli_cat_essen::{Game, VIEW_WIDTH, VIEW_HEIGHT};

pub fn main() -> Result<(), Error> {
    let (mut cam, drawing_area, context) = DrawingArea::new(
        "DSJ 2023",
        Rect::new(0.0, 0.0, VIEW_WIDTH, VIEW_HEIGHT),
        Vec2::new(VIEW_WIDTH * 2.0, VIEW_HEIGHT * 2.0)
    )?;
    let mut render = Render::new(drawing_area, &context)?;
    let mut controls = Controls::new(&context)?;
    let mut game = Game::new(&mut render)?;

    while !controls.should_close {
        controls.update(&cam);
        game.update(&mut controls);
        if game.input.fullscreen.down(true) {
            render.toggle_fullscreen(&mut cam)?;
        }
        render.start_draw();
        game.draw(&mut cam);
        render.end_draw(&mut cam)?;
    }
    
    Ok(())
}
