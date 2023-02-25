use std::collections::VecDeque;

use crate::sandwitch::{Ingredients, SandwitchMachine, Sandwitch, SandwitchRender, get_rand_ingredient};

use nze_game_sdl::{Camera, geometry::Vec2};
use rand::prelude::*;

struct Customer {
    ings: VecDeque<Ingredients>,
    finished: bool,
}

const MIN_REQUEST_SIZE: f64 = 2.0;
const MAX_REQUEST_DELTA: f64 = 4.0;

impl Customer {
    pub fn new() -> Customer {
        Customer {
            ings: VecDeque::new(),
            finished: false,
        }
    }

    pub fn populate(&mut self, rng: &mut ThreadRng) {
        let size = ((rng.gen::<f64>() as f64 * MAX_REQUEST_DELTA) + MIN_REQUEST_SIZE).round() as usize;
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
            if *ing != sw.ingredients[i] { return false; }
        }
        self.finished = true;
        return true;
    }
}

pub struct CustomerLine {
    active_customers: usize,
    customers: Vec<Customer>,
    leaving_customers: Vec<Customer>,
    rng: ThreadRng,
}

impl CustomerLine {
    pub fn new() -> CustomerLine {
        let mut line = CustomerLine {
            active_customers: 2,
            customers: vec![Customer::new(), Customer::new(), Customer::new()],
            leaving_customers: Vec::new(),
            rng: thread_rng(),
        };
        line.populate_customers();
        line
    }

    pub fn update(&mut self, dt: f64) {

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
                if self.customers[i].request_met(sw) {
                    sw.ingredients.clear();
                    self.leaving_customers.push(self.customers.remove(i));
                }
            }
        }
    } 
}

pub struct CustomerRender {
    
}

impl CustomerRender {
    pub fn new() -> CustomerRender {
        CustomerRender {
            
        }
    }
    
    pub fn draw(&self, cam: &mut Camera, customers: &CustomerLine, sw_render: &SandwitchRender) {
        for i in 0..customers.active_customers {
            if customers.customers.len() <= i { break; }
            sw_render.render_sandwitch(cam, customers.customers[i].ings.iter(),
                                       Vec2::new(180.0, 300.0), 33.0 * i as f64, Vec2::new(30.0, 10.0), -2.0, -1.0);
        }
    }
}
