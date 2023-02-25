use nze_game_sdl::input::{Controls, keyboard::Key, controller::Button};

pub struct Btn {
    input: bool,
    prev_input: bool,
    key: Vec<Key>,
    btn: Vec<Button>,
}

impl Btn {
    pub fn new(key: Vec<Key>, btn: Vec<Button>) -> Btn {
        Btn {
            input: false,
            prev_input: false,
            key, btn
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
            left: Btn::new(vec![Key::Left, Key::A], vec![Button::DPadLeft]),
            right: Btn::new(vec![Key::Right, Key::D], vec![Button::DPadRight]),
            down: Btn::new(vec![Key::Down, Key::S], vec![Button::DPadRight]),
            up: Btn::new(vec![Key::Up, Key::W], vec![Button::DPadUp]),
        }
    }

    pub fn update(&mut self, controls: &Controls) {
        self.left.update(controls);
        self.right.update(controls);
        self.down.update(controls);
        self.up.update(controls);
    }
}
