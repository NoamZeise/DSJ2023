use std::{collections::{VecDeque, HashMap, vec_deque::Iter}, path::Path, slice::IterMut};

use nze_game_sdl::{Render, Camera, Error, GameObject, geometry::Vec2};
use rand::prelude::*;

use crate::moving_target::Target;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum Ingredients {
    Bread,
    Lettuce,
    Meat,
    Tomato,
}

pub const INGREDIENT_COUNT: usize = 4;

pub fn get_ingredient(index: usize) -> Ingredients{
    match index {
        0 => Ingredients::Bread,
        1 => Ingredients::Lettuce,
        2 => Ingredients::Meat,
        3 => Ingredients::Tomato,
        _ => panic!("index of ingredient out of range"),
    }
}

pub fn get_rand_ingredient(rng: &mut ThreadRng) -> Ingredients {
    get_ingredient((rng.gen::<f64>() * INGREDIENT_COUNT as f64) as usize)
}

#[derive(Clone)]
pub struct Sandwitch {
    pub ingredients : VecDeque<Ingredients>,
    pub ing_targets: VecDeque<Target>,
    pub target: Target,
    sw_dir: f64,
}

impl Sandwitch {
    pub fn new() -> Sandwitch {
        Sandwitch {
            ingredients: VecDeque::new(),
            ing_targets: VecDeque::new(),
            target: Target::new(),
            sw_dir: -1.0,
        }
    }

    pub fn add_back(&mut self, ingredient: Ingredients, t: Target) {
        self.ingredients.push_back(ingredient);
        let mut t = t;
        t.speed = self.target.speed;
        self.ing_targets.push_back(t);
    }

    pub fn add(&mut self, ingredient: Ingredients, rng: &mut ThreadRng) {
        self.ingredients.push_front(ingredient);
        self.ing_targets.push_front(Target::new_with_speed(
            self.target.speed,
            Vec2::new(
                self.target.get_pos_no_offset().x,
                self.target.get_pos_no_offset().y + Self::get_ing_yoff(self.sw_dir, self.ing_targets.len() as f64 - 10.0)
            ),
            Vec2::new((rng.gen::<f64>() * 3.0) - 1.5, 0.0)
        ));
    }

    pub fn take(&mut self) -> Option<(Ingredients, Target)> {
        let ing = self.ingredients.pop_back();
        let tar = self.ing_targets.pop_back();
        if ing.is_none() || tar.is_none() {
            return None;
        }
        Some((ing.unwrap(), tar.unwrap()))
    }
        

    pub fn reset(&mut self) {
        self.ingredients.clear();
        self.ing_targets.clear();
        self.add_back(Ingredients::Bread, self.target);
    }

    pub fn clear(&mut self) {
        let mut i = 0;
        while i < self.ing_targets.len() && i < self.ingredients.len() {
            if self.ing_targets[i].is_active() {
                self.ing_targets.remove(i);
                self.ingredients.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn set_target(&mut self, target: Vec2) {
        self.target.set_target(target);
        for (i, t) in self.ing_targets.iter_mut().enumerate() {
            t.set_target(
                Vec2::new(
                    target.x,
                    target.y + Self::get_ing_yoff(self.sw_dir, i as f64)
                )
            );
        }
    }

    fn get_ing_yoff(dir: f64, i: f64) -> f64 {
       dir * (i as f64 * (ING_SIZE.y + QUEUE_ING_SPACING))
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.target.speed = speed;
        for t in self.ing_targets.iter_mut() {
            t.speed = speed;
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.target.update(dt);
        for t in self.ing_targets.iter_mut() {
            t.update(dt);
        }
    }
}

const QUEUE_SPEED: f64 = 400.0;

pub struct SandwitchMachine {
    sandwitches: Vec<Sandwitch>,
    queue_size: usize,
    queue: Sandwitch,
    active: usize,
    rng: ThreadRng,
    delicat_target: Target,
}

impl SandwitchMachine {
    pub fn new() -> SandwitchMachine {
        let mut sm = SandwitchMachine {
            queue_size: 6,
            sandwitches: Vec::new(),
            queue: Sandwitch::new(),
            active: 0,
            rng:thread_rng(),
            delicat_target: Target::new(),
        };
        sm.delicat_target.breath = true;
        sm.delicat_target.set_target(DELICAT_LOCATION);
        sm.delicat_target.breath_speed /= 2.5;
        sm.delicat_target.breath_size.y = 1.5;
        for _ in 0..sm.queue_size {
            sm.sandwitches.push(Sandwitch::new());
        }
        for s in sm.sandwitches.iter_mut() {
            s.reset();
        }
        sm.queue.set_speed(QUEUE_SPEED);
        sm.queue.set_target(sm.get_queue_target());
        sm.queue.target.breath = true;
        sm.queue.target.breath_size = Vec2::new(0.0, 4.0);
        sm.queue.target.breath_speed = 0.5;
        sm.queue.sw_dir = 1.0;
        sm.fill_queue();
        sm
    }

    fn get_queue_target(&self) -> Vec2 {
        Vec2::new(QUEUE_BASE.x + QUEUE_MOVE * self.active as f64,
                  QUEUE_BASE.y
        )
    }

    pub fn update(&mut self, dt: f64) {
        for (i, sm) in self.sandwitches.iter_mut().enumerate() {
            sm.update(dt);
            sm.set_target(
                Vec2::new(SANDWITCH_BASE.x + (i as f64 * QUEUE_MOVE), SANDWITCH_BASE.y)
            );
        }
        self.sandwitches[0].clear();
        self.queue.set_target(self.get_queue_target());
        self.queue.update(dt);
        self.queue.target.breath_update(dt);
        self.delicat_target.breath_update(dt);
    }

    pub fn fill_queue(&mut self) {
        while self.queue.ingredients.len() < self.queue_size {
            self.queue.add(get_rand_ingredient(&mut self.rng), &mut self.rng);
        }
    }

    pub fn release(&mut self) {
        let (i, t) = self.queue.take().unwrap();
        self.sandwitches[self.active].add_back(i, t);
        self.fill_queue();
    }

    pub fn bin(&mut self) {
        if self.sandwitches[self.active].ingredients.len() > 0 {
           // if self.sandwitches[self.active].ingredients.len() == 1 && self.active != 0 {
           //     return;
           // }
            let (i, t) = self.sandwitches[self.active].take().unwrap();
            self.queue.add_back(
                i, t
            );
        }
    }

    pub fn switch(&mut self, diff: i32) {
        let new = self.active as i32 + diff;
        if new < 0 {
            self.active = self.sandwitches.len() - new.abs() as usize;
        } else {
            self.active = new as usize % self.sandwitches.len();
        }
    }

    pub fn sandwitches(&mut self) -> IterMut<Sandwitch> {
        self.sandwitches.iter_mut()
    }
    
}

pub struct SandwitchRender {
    ingredient: HashMap<Ingredients, GameObject>,
    chef: GameObject,
    restauraunt_front: GameObject,
    delicat: GameObject,
    plate: GameObject,
}

pub const ING_SIZE: Vec2 = Vec2::new(46.0, 24.0);
const QUEUE_BASE: Vec2 = Vec2::new(30.0, 20.0);
const CHEF_OFFSET: Vec2 = Vec2::new(-20.0, 100.0);
const QUEUE_MOVE: f64 = ING_SIZE.x * 1.6;
const QUEUE_ING_SPACING: f64 = -ING_SIZE.y * 0.5;
pub const SANDWITCH_BASE: Vec2 = Vec2::new(QUEUE_BASE.x, QUEUE_BASE.y + ING_SIZE.y * 8.0);

const DELICAT_LOCATION: Vec2 = Vec2::new(0.0, 210.0);
const PLATE_OFFSET: Vec2 = Vec2::new(-2.0, 11.0);

impl SandwitchRender {
    pub fn new(render: &mut Render) -> Result<SandwitchRender, Error> {
        Ok(SandwitchRender {
            ingredient: Self::get_ingredient_hash(render)?,
            chef: GameObject::new_from_tex(render.texture_manager.load(Path::new("resources/textures/chef.png"))?),
            delicat: GameObject::new_from_tex(render.texture_manager.load(Path::new("resources/textures/deli-cat.png"))?),
            restauraunt_front: GameObject::new_from_tex(render.texture_manager.load(
                Path::new("resources/textures/restaurant_front.png"))?),
            plate: GameObject::new_from_tex(render.texture_manager.load(
                Path::new("resources/textures/plate.png"))?),
        })
    }

    pub fn draw(&mut self, cam: &mut Camera, machine: &SandwitchMachine) {
        self.chef.rect.x = machine.queue.target.get_pos().x + CHEF_OFFSET.x;
        self.chef.rect.y = machine.queue.target.get_pos().y + CHEF_OFFSET.y;
        cam.draw(&self.chef);
        cam.draw(&self.restauraunt_front);
        let delicat_pos = machine.delicat_target.get_pos();
        self.delicat.rect.x = delicat_pos.x;
        self.delicat.rect.y = delicat_pos.y;
        cam.draw(&self.delicat);
        self.render_sw(cam, &machine.queue);

        for (i, sw) in machine.sandwitches.iter().enumerate() {
            if i != 0 {
                let mut p = self.plate.clone();
                let pos = sw.target.get_pos();
                p.rect.x = pos.x + PLATE_OFFSET.x;
                p.rect.y = pos.y + PLATE_OFFSET.y;
                cam.draw(&p);
            }
            self.render_sw(cam, &sw);
        }
    }

    pub fn render_sw(&self, cam: &mut Camera, sw: &Sandwitch) {
        for (i, ing) in sw.ingredients.iter().enumerate() {
            let mut ing = self.ingredient.get(&ing).unwrap().clone();
            ing.rect.w = ING_SIZE.x;
            ing.rect.h = ING_SIZE.y;
            ing.rect.x = sw.ing_targets[i].get_pos_no_offset().x;
            ing.rect.y = sw.ing_targets[i].get_pos_no_offset().y;
            cam.draw(&ing);
        }
    }

    pub fn render_ings(&self, cam: &mut Camera, ings: Iter<Ingredients>,
                            base: Vec2, x_off: f64, ing_size: Vec2, ing_spacing: f64, dir_mod: f64
    ) {
        for (i, ing) in ings.enumerate() {
            let mut ing = self.ingredient.get(&ing).unwrap().clone();
            ing.rect.w = ing_size.x;
            ing.rect.h = ing_size.y;
            ing.rect.x = base.x + x_off;
            ing.rect.y = base.y + dir_mod * (i as f64 * (ing_size.y + ing_spacing));
            cam.draw(&ing);
        }
    }

    fn get_ingredient_hash(render: &mut Render) -> Result<HashMap<Ingredients, GameObject>, Error> {
        let mut textures = HashMap::new();
        Self::add_ing_to_hashmap(
            render, &mut textures,
            Ingredients::Bread, Path::new("resources/textures/ingredient/bread.png"))?;
        Self::add_ing_to_hashmap(
            render, &mut textures,
            Ingredients::Lettuce, Path::new("resources/textures/ingredient/lettuce.png"))?;
        Self::add_ing_to_hashmap(
            render, &mut textures,
            Ingredients::Meat, Path::new("resources/textures/ingredient/patty.png"))?;
        Self::add_ing_to_hashmap(
            render, &mut textures,
            Ingredients::Tomato, Path::new("resources/textures/ingredient/tomato.png"))?;
        Ok(textures)
    }

    fn add_ing_to_hashmap(render: &mut Render, ingredients: &mut HashMap<Ingredients, GameObject>,
                          ing: Ingredients, res: &Path) -> Result<(), Error> {
        ingredients.insert(ing,
                           GameObject::new_from_tex(render.texture_manager.load(res)?));
        Ok(())
    }
}
