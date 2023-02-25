use nze_game_sdl::geometry::Vec2;


#[derive(Clone, Copy)]
pub struct Target {
    current_pos: Vec2,
    target_pos: Vec2,
    pub speed: f64,
    pub breath: bool,
    pub breath_size: Vec2,
    pub breath_speed: f64,
    time: f64,
    offset: Vec2,
}

impl Target {
    pub fn new() -> Target {
        Self::new_with_speed(100.0, Vec2::zero())
    }

    pub fn new_with_speed(speed: f64, target: Vec2) -> Target {
        Target {
            current_pos: target,
            target_pos: target,
            speed,
            breath: false,
            breath_size: Vec2::new(1.0, 1.5),
            offset: Vec2::zero(),
            time: 0.0,
            breath_speed: 2.0,
        }
    }
    
    pub fn set_target(&mut self, target: Vec2) {
        self.target_pos = target;
        if self.current_pos == Vec2::zero() {
            self.current_pos = self.target_pos;
        }
    }

    pub fn is_active(&self) -> bool {
        self.current_pos == self.target_pos
    }

    pub fn update(&mut self, dt: f64) {
        if  self.current_pos == self.target_pos {
            return;
        }
        let mv = self.target_pos - self.current_pos;
        let mag = (mv.x * mv.x + mv.y * mv.y).sqrt();
        let mv = mv / mag;
        self.current_pos += mv * dt * self.speed;
        if mag < 1.0 * (self.speed / 100.0){
            self.current_pos = self.target_pos;
        }
    }

    pub fn breath_update(&mut self, dt: f64) {
        if self.breath {
            self.time += dt;
            self.offset = self.breath_size * (self.time * self.breath_speed).sin();
        }
    }

    pub fn get_pos(&self) -> Vec2 {
        self.current_pos + self.offset
    }

    pub fn get_pos_no_offset(&self) -> Vec2 {
        self.current_pos
    }
}
