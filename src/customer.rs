use std::{collections::VecDeque, path::Path};

use crate::{sandwitch::{Ingredients, SandwitchMachine, Sandwitch, SandwitchRender, get_rand_ingredient}, moving_target::Target};

use nze_game_sdl::{Camera, geometry::{Vec2, Rect}, GameObject, Render, Error, Colour};
use rand::prelude::*;

struct Customer {
    ings: VecDeque<Ingredients>,
    sandwitch: Option<Sandwitch>,
    finished: bool,
    target: Target,
    waiting: bool,
    wait_max: f64,
    wait_time: f64,
    max_request_delta: f64,
}

const MIN_REQUEST_SIZE: f64 = 2.0;
const MAX_REQUEST_DELTA: f64 = 3.0;
const INITIAL_WAIT_TIME: f64 = 45.0;
const INITIAL_SPAWN_TIME: f64 = 15.0;
pub const INITIAL_LIVES: u32 = 3;
const ACTIVE_CUSTOMERS: usize = 3;

impl Customer {
    
    pub fn new(score: f64) -> Customer {
       let mut c =  Customer {
           ings: VecDeque::new(),
           sandwitch: None,
           finished: false,
           target: Target::new(),
           waiting: false,
           wait_time: 0.0,
           wait_max: (INITIAL_WAIT_TIME - (score as f64 * 0.8)).max(15.0),
           max_request_delta: MAX_REQUEST_DELTA + (score * 0.2),
       };
        c.target.breath = true;
        c
    }

    pub fn populate(&mut self, rng: &mut ThreadRng) {
        let size = ((rng.gen::<f64>() as f64 * self.max_request_delta) + MIN_REQUEST_SIZE).round() as usize;
        for i in 0..size {
            if i == 0 || i == size - 1 {
                self.ings.push_front(Ingredients::Bread);
            } else {
                let mut ing = get_rand_ingredient(rng);
                if ing == Ingredients::Bread {
                    ing = get_rand_ingredient(rng);
                }
                self.ings.push_front(ing);
            }
        }
    }

    pub fn request_met(&mut self, sw: &Sandwitch) -> bool {
        if sw.ingredients.len() != self.ings.len() {
            return false;
        }
        for (i, ing) in self.ings.iter().enumerate() {
            if !sw.ing_targets[i].is_active() {
                return false;
            }
            if *ing != sw.ingredients[i] { return false; }
        }
        self.finished = true;
        return true;
    }

    pub fn update(&mut self, dt: f64) {
        self.target.update(dt);
        if self.waiting {
            self.wait_time += dt;
        }
    }

    pub fn waited_too_long(&self) -> bool {
        self.wait_time > self.wait_max
    }
}

const MAX_CUSTOMERS: usize = 6;

pub struct CustomerLine {
    active_customers: usize,
    customers: Vec<Customer>,
    leaving_customers: Vec<Customer>,
    angry_customers: Vec<Customer>,
    rng: ThreadRng,
    time_since_customer: f64,
    next_customer_delay: f64,
    score: u64,
    lives: u32,
}

impl CustomerLine {
    pub fn new() -> CustomerLine {
        let mut line = CustomerLine {
            active_customers: ACTIVE_CUSTOMERS,
            customers: vec![],
            leaving_customers: Vec::new(),
            angry_customers: Vec::new(),
            rng: thread_rng(),
            time_since_customer: INITIAL_SPAWN_TIME / 2.0,
            next_customer_delay: INITIAL_SPAWN_TIME,
            score: 0,
            lives: INITIAL_LIVES,
        };
        line.populate_customers();
        line
    }

    fn add_customer(&mut self) {
        self.customers.push(Customer::new(self.score as f64));
        self.customers.last_mut().unwrap().target.breath_speed = self.rng.gen::<f64>() * 0.1 + 1.0;
        self.customers.last_mut().unwrap().target.breath_size.y = self.rng.gen::<f64>() * 0.1 + 1.0;
        self.populate_customers();
    }

    pub fn update(&mut self, dt: f64) {
        self.time_since_customer += dt;
        if self.time_since_customer > self.next_customer_delay && self.customers.len() < MAX_CUSTOMERS {
            self.time_since_customer = 0.0;
            self.add_customer();
        }
        let mut to_remove = None;
        for (i, c) in self.customers.iter_mut().enumerate() {
            if c.waiting {
                c.target.breath_update(dt);
            }
            if c.waiting && self.leaving_customers.len() > 0 { continue; }
            if c.target.get_pos() == Vec2::zero() {
                c.target.set_target(CUSTOMER_START)
            }
            c.target.set_target(Vec2::new(
                CUSTOMER_BASE.x + CUSTOMER_OFFSET.x + (
                    CUSTOMER_SIZE.x *
                        if c.waiting { i } else { i + self.leaving_customers.len() } as f64
                ),
                CUSTOMER_BASE.y + CUSTOMER_OFFSET.y
            ));
            if c.target.is_active() {
                c.waiting = true;
            }
            c.update(dt);
            if c.waited_too_long() {
                to_remove = Some(i);
            }
        }

        match to_remove {
            Some(i) => {
                self.angry_customers.push(self.customers.remove(i));
                self.populate_customers();
            },
            None => (),
        }

        let mut c_i = 0;
        while c_i < self.leaving_customers.len() {
            self.leaving_customers[c_i].target.set_target(
                CUSTOMER_END
            );
            let mut target = self.leaving_customers[c_i].target.get_pos();
            target.y += 30.0;
            self.leaving_customers[c_i].sandwitch.as_mut().unwrap().set_target(
                target
            );
            self.leaving_customers[c_i].sandwitch.as_mut().unwrap().update(dt);
            if self.leaving_customers[c_i].sandwitch.as_mut().unwrap().target.is_active() {
                self.leaving_customers[c_i].target.update(dt);
            }
            if self.leaving_customers[c_i].target.is_active() {
                self.leaving_customers.remove(c_i);
            } else {
                c_i += 1;
            }
        }

        let mut angry_i = 0;
        while angry_i < self.angry_customers.len() {
            self.angry_customers[angry_i].target.set_target(CUSTOMER_END);
            self.angry_customers[angry_i].update(dt);
            if self.angry_customers[angry_i].target.is_active() {
                self.angry_customers.remove(angry_i);
                if self.lives > 0 {
                    self.lives -= 1;
                }
            } else  {
                angry_i += 1;
            }
        }
    }

    fn populate_customers(&mut self) {
        for i in 0..self.active_customers {
            if self.customers.len() <= i { break; }
            if self.customers[i].ings.len() == 0 {
                self.customers[i].populate(&mut self.rng);
            }
        }
    }
    
    pub fn check_machine(&mut self, machine: &mut SandwitchMachine) {
        for sw in machine.sandwitches() {
            for i in 0..self.active_customers {
                if self.customers.len() <= i { break; }
                if self.customers[i].waiting {
                    if self.customers[i].request_met(sw) {
                        self.leaving_customers.push(self.customers.remove(i));
                        self.leaving_customers.last_mut().unwrap().sandwitch = Some(sw.clone());
                        sw.reset();
                        self.populate_customers();
                        self.add_score();
                    }
                }
            }
        }
    }

    fn add_score(&mut self) {
        self.score += 1;
        self.next_customer_delay = (INITIAL_SPAWN_TIME - (self.score as f64 * 0.4)).max(5.0);
    }

    pub fn get_score(&self) -> u64 {
        self.score
    }

    pub fn lives(&self) -> u32 {
        self.lives
    }
}

pub struct CustomerRender {
    customer: GameObject,
    speech: GameObject,
}
const CUSTOMER_START: Vec2 = Vec2::new(500.0, 280.0);
const CUSTOMER_BASE: Vec2 = Vec2::new(100.0, CUSTOMER_START.y - 5.0);
const CUSTOMER_END: Vec2 = Vec2::new(CUSTOMER_BASE.x - 25.0, CUSTOMER_BASE.y + 150.0);
const CUSTOMER_ING_SIZE: Vec2 = Vec2::new(24.0, 12.0);
const CUSTOMER_ING_OFFSET: Vec2 = Vec2::new(27.0, 30.0);
const CUSTOMER_ING_SPACING: f64 = -CUSTOMER_ING_SIZE.y * 0.5;
const CUSTOMER_SIZE: Vec2 = Vec2::new(90.0, 0.0);
const CUSTOMER_OFFSET: Vec2 = Vec2::new(-20.0, 20.0);

const CUSTOMER_SPEECH_OFFSET: Vec2 = Vec2::new(2.0, 25.0);

const CUSTOMER_PATIENCE_OFFSET: Rect = Rect::new(15.0, -5.0, 30.0, 5.0);

impl CustomerRender {
    pub fn new(render: &mut Render) -> Result<CustomerRender, Error> {
        Ok(CustomerRender {
            customer: GameObject::new_from_tex(
                render.texture_manager.load(
                    Path::new("resources/textures/customer.png"))?
            ),
            speech: GameObject::new_from_tex(
                render.texture_manager.load(
                    Path::new("resources/textures/speech.png"))?
            ),
        })
    }
    
    pub fn draw(&self, cam: &mut Camera, customers: &mut CustomerLine, sw_render: &SandwitchRender) {
        for c in customers.customers.iter_mut() {
            self.draw_customer(cam, c);
            if c.waiting {
                self.draw_patience(cam, c);
            }
        }
        for i in 0..customers.active_customers {
            if customers.customers.len() <= i { break; }
            if !customers.customers[i].waiting {
                continue;
            }
            let pos_abs = customers.customers[i].target.get_pos_no_offset();
            let mut speech = self.speech.clone();
            speech.rect.x = pos_abs.x - CUSTOMER_OFFSET.x + CUSTOMER_ING_OFFSET.x + CUSTOMER_SPEECH_OFFSET.x;
            speech.rect.h = (customers.customers[i].ings.len() as f64 * (CUSTOMER_ING_SIZE.y + CUSTOMER_ING_SPACING)) + 25.0;
            speech.rect.y = (pos_abs.y - CUSTOMER_OFFSET.y - speech.rect.h) + CUSTOMER_SPEECH_OFFSET.y + CUSTOMER_ING_OFFSET.y;
            cam.draw(&speech);
            sw_render.render_ings(cam, customers.customers[i].ings.iter(),
                                  Vec2::new(pos_abs.x - CUSTOMER_OFFSET.x,
                                            pos_abs.y - CUSTOMER_OFFSET.y + CUSTOMER_ING_OFFSET.y),
                                       CUSTOMER_ING_OFFSET.x + 13.0,
                                       CUSTOMER_ING_SIZE, CUSTOMER_ING_SPACING, -1.0);
        }
        for c in customers.leaving_customers.iter_mut() {
            self.draw_customer(cam, c);
            sw_render.render_sw(cam, c.sandwitch.as_ref().unwrap())
        }
        for c in customers.angry_customers.iter() {
            self.draw_customer(cam, c);
        }
    }

    fn draw_customer(&self, cam: &mut Camera, c: &Customer) {
        let mut go = self.customer.clone();
        let pos = c.target.get_pos();
        go.rect.x = pos.x;
        go.rect.y = pos.y;
        cam.draw(&go);
    }

    fn draw_patience(&self, cam: &mut Camera, c: &Customer) {
        let mut pos = c.target.get_pos();
        pos.x += CUSTOMER_PATIENCE_OFFSET.x;
        pos.y += CUSTOMER_PATIENCE_OFFSET.y;
        let ratio = c.wait_time / c.wait_max;
        let length = CUSTOMER_PATIENCE_OFFSET.w * ratio;
        cam.draw_rect(Rect::new(pos.x, pos.y, CUSTOMER_PATIENCE_OFFSET.w, CUSTOMER_PATIENCE_OFFSET.h),
                      Colour::new(124, 199, 109, 255), Vec2::new(1.0, 1.0));
        cam.draw_rect(Rect::new(pos.x, pos.y, length, CUSTOMER_PATIENCE_OFFSET.h),
                      Colour::new(117, 68, 68, 255), Vec2::new(1.0, 1.0));
    }
}
