use std::path::Path;

use customer::{CustomerLine, CustomerRender};
use nze_game_sdl::{
    Camera,
    Render,
    input::Controls,
    Error, GameObject, resource::Font, Colour, geometry::{Vec2, Rect},
};

mod sandwitch;
mod customer;
pub mod input;
mod moving_target;

use sandwitch::{SandwitchMachine, SandwitchRender};
use input::Input;

pub const VIEW_WIDTH: f64  = 480.0;
pub const VIEW_HEIGHT: f64 = 360.0;

pub struct Game {
    machine: SandwitchMachine,
    pub input: Input,
    sandwitch_render: SandwitchRender,
    customer_line: CustomerLine,
    customer_render: CustomerRender,
    bg: GameObject,
    font: Font,
    game_ended: bool,
    end_screen: GameObject,
    end_sign: GameObject,
    paused: bool,
}

impl Game {
    pub fn new(render: &mut Render) -> Result<Game, Error> {
        Ok(Game {
            machine: SandwitchMachine::new(),
            input: Input::new(),
            sandwitch_render: SandwitchRender::new(render)?,
            customer_line: CustomerLine::new(),
            customer_render: CustomerRender::new(render)?,
            bg: GameObject::new_from_tex(render.texture_manager.load(Path::new("resources/textures/restaurant.png"))?),
            font: render.font_manager.load_font(Path::new("resources/fonts/ShortStack-Regular.ttf"))?,
            end_screen: GameObject::new_from_tex(render.texture_manager.load(Path::new("resources/textures/end.png"))?),
            end_sign: GameObject::new_from_tex(render.texture_manager.load(Path::new("resources/textures/sign.png"))?),
            game_ended: false,
            paused: false,
        })
    }

    pub fn update(&mut self, controls: &mut Controls) {
        self.input.update(controls);
        if self.input.pause.down(true) {
            self.paused = !self.paused;
        }
        if self.game_ended {
            if self.end_screen.rect.y == 0.0 {
                if self.input.down.down(true) {
                    self.customer_line = CustomerLine::new();
                    self.machine = SandwitchMachine::new();
                    self.game_ended = false;
                }
            } else {
                self.end_screen.rect.y += controls.frame_elapsed * 100.0;
                if self.end_screen.rect.y > 0.0 {
                    self.end_screen.rect.y = 0.0;
                }
            }
        }
        else if ! self.paused {
            self.game_update(controls);
        }
    }

    fn game_update(&mut self, controls: &mut Controls) {
        self.customer_line.update(controls.frame_elapsed);
        self.machine.update(controls.frame_elapsed);
        self.customer_line.check_machine(&mut self.machine);
        if self.input.left.down(true) {
            self.machine.switch(-1);
        }
        if self.input.right.down(true) {
            self.machine.switch(1);
        }
        if self.input.down.down(true) {
            self.machine.release();
        }
        if self.input.up.down(true) {
            self.machine.bin();
        }
        if self.customer_line.lives() == 0 {
            self.game_ended = true;
            self.paused = false;
            self.end_screen.rect.y = - VIEW_HEIGHT;
        }
    }
    
    pub fn draw(&mut self, cam: &mut Camera) {
        cam.draw(&self.bg);
        cam.draw_rect(self.bg.rect, Colour::new(0, 0, 0, BG_OPACITY), Vec2::zero());
       // cam.draw_rect(Rect::new(SCORE_POS.x - 5.0, SCORE_POS.y - 5.0, 120.0, SCORE_SIZE as f64 * 1.5),
        //              Colour::new(100, 100, 100, 255), Vec2::new(1.0, 1.0));
        cam.draw_disposable_text(&self.font, format!("Score: {}", self.customer_line.get_score()), SCORE_SIZE, SCORE_POS,
                                 Colour::new(100, 100, 10, 255), Vec2::new(1.0, 1.0));
        for i in 0..self.customer_line.lives() {
            cam.draw_rect(Rect::new(LIVES_POS.x + ((LIVES_RECT.x + LIVES_BUFFER) * i as f64),
                                    LIVES_POS.y, LIVES_RECT.x, LIVES_RECT.y),
                          Colour::new(255, 0, 0, 255), Vec2::new(0.0, 0.0));
        }
        
        self.sandwitch_render.draw(cam, &self.machine);
        self.customer_render.draw(cam, &mut self.customer_line, &self.sandwitch_render);
        if self.game_ended {
            cam.draw(&self.end_screen);
            cam.draw_disposable_text(&self.font, format!("Score: {}", self.customer_line.get_score()),
                                     FINAL_SCORE_SIZE * 4,
                                     Vec2::new(self.end_screen.rect.x + 50.0,
                                               self.end_screen.rect.y + 120.0),
                                     Colour::new(0, 0, 0, 255), Vec2::new(1.0, 1.0));
            cam.draw_disposable_text(&self.font, "Press Down To Play Again".to_string(),
                                     FINAL_SCORE_SIZE,
                                     Vec2::new(self.end_screen.rect.x + 70.0,
                                               self.end_screen.rect.y + 270.0),
                                     Colour::new(0, 0, 0, 255), Vec2::new(1.0, 1.0));
            cam.draw(&self.end_sign);
        }

        if self.paused {
            cam.draw_rect(Rect::new(0.0, 0.0, VIEW_WIDTH, VIEW_HEIGHT),
                          Colour::new(0, 0, 0, 100), Vec2::zero());
            cam.draw_disposable_text(&self.font, "Pawsed".to_string(), FINAL_SCORE_SIZE * 4,
                                     Vec2::new(60.0, 120.0), Colour::white(), Vec2::zero()
            );
        }
    }
}

const SCORE_POS: Vec2 = Vec2::new(160.0, 80.0);
const SCORE_SIZE: u32 = 40;
const FINAL_SCORE_SIZE: u32 = 25;

const LIVES_RECT: Vec2 = Vec2::new(10.0, 10.0);
const LIVES_BUFFER: f64 = 10.0;
const LIVES_POS: Vec2 = Vec2::new(SCORE_POS.x + 40.0, SCORE_POS.y + LIVES_RECT.y * 4.0);

const BG_OPACITY: u8 = 64;
