use customer::{CustomerLine, CustomerRender};
use nze_game_sdl::{
    Camera,
    Render,
    input::Controls,
    input::keyboard::Key,
    Error,
};

mod sandwitch;
mod customer;
mod input;

use sandwitch::{SandwitchMachine, SandwitchRender};
use input::Input;

pub const VIEW_WIDTH: f64  = 480.0;
pub const VIEW_HEIGHT: f64 = 360.0;

pub struct Game {
    machine: SandwitchMachine,
    input: Input,
    sandwitch_render: SandwitchRender,
    customer_line: CustomerLine,
    customer_render: CustomerRender,
}

impl Game {
    pub fn new(render: &mut Render) -> Result<Game, Error> {
        Ok(Game {
            machine: SandwitchMachine::new(),
            input: Input::new(),
            sandwitch_render: SandwitchRender::new(render)?,
            customer_line: CustomerLine::new(),
            customer_render: CustomerRender::new(),
        })
    }

    pub fn update(&mut self, controls: &mut Controls) {
        self.input.update(controls);
        
        if controls.kb.press(Key::Escape) {
            controls.should_close = true;
        }
        if self.input.left.down(true) {
            self.machine.switch(-1);
        }
        if self.input.right.down(true) {
            self.machine.switch(1);
        }
        if self.input.down.down(true) {
            self.machine.release();
            self.customer_line.check_machine(&mut self.machine);
        }
        if self.input.up.down(true) {
            self.machine.bin();
        }
    }
    pub fn draw(&mut self, cam: &mut Camera) {
        self.sandwitch_render.draw(cam, &self.machine);
        self.customer_render.draw(cam, &self.customer_line, &self.sandwitch_render);
    }
}
