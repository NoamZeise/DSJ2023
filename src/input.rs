use nze_game_sdl::input::{Controls, keyboard::Key, controller::{Button, Side}};

enum Dir {
    Up,
    Down,
    Left,
    Right
}

struct Joy {
    dir: Dir,
    side: Side,
}

const JOY_ACTIVATION: f64 = 0.8;
impl Joy {
    fn new(side: Side, dir: Dir) -> Joy{
        Joy { dir, side }
    }
    fn check(&self, controls: &Controls) -> bool {
        let v =  controls.c.joy(0, self.side.clone());
        match self.dir {
            Dir::Up => v.y < -JOY_ACTIVATION,
            Dir::Down => v.y > JOY_ACTIVATION,
            Dir::Left => v.x < -JOY_ACTIVATION,
            Dir::Right => v.x > JOY_ACTIVATION,
        }
    }
}

pub struct Btn {
    input: bool,
    prev_input: bool,
    key: Vec<Key>,
    btn: Vec<Button>,
    joy: Vec<Joy>,
}

impl Btn {
    fn new(key: Vec<Key>, btn: Vec<Button>, joy: Vec<Joy>) -> Btn {
        Btn {
            input: false,
            prev_input: false,
            key, btn, joy
        }
    }

    pub fn update(&mut self, controls: &Controls) {
        self.prev_input = self.input;
        self.input = false;
        for k in self.key.iter() {
            if controls.kb.down(*k) {
                self.input = true;
            }
        }
        for b in self.btn.iter() {
            if controls.c.hold(0, *b) {
                self.input = true;
            }
        }
        for j in self.joy.iter() {
            if j.check(controls) {
                self.input = true;
            }
        }
    }

    pub fn down(&self, press: bool) -> bool {
        if press {
            self.input && !self.prev_input
        } else {
            self.input
        }
    }
}

pub struct Input {
    pub left: Btn,
    pub right: Btn,
    pub down: Btn,
    pub up: Btn,
}

impl Input {
    pub fn new() -> Input {
        Input {
            left: Btn::new(
                vec![Key::Left, Key::A],
                vec![Button::DPadLeft],
                vec![Joy::new(Side::Left, Dir::Left)]
            ),
            right: Btn::new(
                vec![Key::Right, Key::D],
                vec![Button::DPadRight],
                vec![Joy::new(Side::Left, Dir::Right)]
            ),
            down: Btn::new(
                vec![Key::Down, Key::S],
                vec![Button::DPadDown, Button::B],
                vec![Joy::new(Side::Left, Dir::Down)]
            ),
            up: Btn::new(
                vec![Key::Up, Key::W],
                vec![Button::DPadUp, Button::A],
                vec![Joy::new(Side::Left, Dir::Up)]
            ),
        }
    }

    pub fn update(&mut self, controls: &Controls) {
        self.left.update(controls);
        self.right.update(controls);
        self.down.update(controls);
        self.up.update(controls);
    }
}
