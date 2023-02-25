use std::{collections::{VecDeque, HashMap, vec_deque::Iter}, path::Path, slice::IterMut};

use nze_game_sdl::{Render, Camera, Error, GameObject, geometry::Vec2};
use rand::prelude::*;

#[derive(Hash, Eq, PartialEq)]
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

pub struct Sandwitch {
    pub ingredients : VecDeque<Ingredients>,
}

impl Sandwitch {
    pub fn new() -> Sandwitch {
        Sandwitch {
            ingredients: VecDeque::new(),
        }
    }

    pub fn add_back(&mut self, ingredient: Ingredients) {
        self.ingredients.push_back(ingredient);
    }

    pub fn add(&mut self, ingredient: Ingredients) {
        self.ingredients.push_front(ingredient);
    }

    pub fn take(&mut self) -> Option<Ingredients> {
        self.ingredients.pop_back()
    }
}

pub struct SandwitchMachine {
    sandwitches: Vec<Sandwitch>,
    queue_size: usize,
    queue: Sandwitch,
    active: usize,
    rng: ThreadRng,
}

impl SandwitchMachine {
    pub fn new() -> SandwitchMachine {
        let mut sm = SandwitchMachine {
            queue_size: 4,
            sandwitches: vec![Sandwitch::new(), Sandwitch::new(), Sandwitch::new(), Sandwitch::new()],
            queue: Sandwitch::new(),
            active: 0,
            rng:thread_rng(),
        };
        sm.fill_queue();
        sm
    }

    pub fn fill_queue(&mut self) {
        while self.queue.ingredients.len() < self.queue_size {
            self.queue.add(get_rand_ingredient(&mut self.rng));
        }
    }

    pub fn release(&mut self) {
        self.sandwitches[self.active].add_back(
            match self.queue.take() {
                Some(i) => i,
                None => Ingredients::Bread,
            }
        );
        self.fill_queue();
    }

    pub fn bin(&mut self) {
        self.sandwitches[self.active].ingredients.clear();
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
}

const ING_SIZE: Vec2 = Vec2::new(50.0, 20.0);
const QUEUE_BASE: Vec2 = Vec2::new(10.0, 20.0);
const QUEUE_MOVE: f64 = ING_SIZE.x * 1.5;
const QUEUE_ING_SPACING: f64 = -ING_SIZE.y * 0.2;
const SANDWITCH_BASE: Vec2 = Vec2::new(QUEUE_BASE.x, QUEUE_BASE.y + ING_SIZE.y * 10.0);

impl SandwitchRender {
    pub fn new(render: &mut Render) -> Result<SandwitchRender, Error> {
        Ok(SandwitchRender {
            ingredient: Self::get_ingredient_hash(render)?,
        })
    }

    pub fn draw(&self, cam: &mut Camera, machine: &SandwitchMachine) {
        self.render_sandwitch(cam, machine.queue.ingredients.iter(),
                              QUEUE_BASE, QUEUE_MOVE * machine.active as f64, ING_SIZE, QUEUE_ING_SPACING, 1.0);

        for (sw_i, sw) in machine.sandwitches.iter().enumerate() {
            self.render_sandwitch(cam, sw.ingredients.iter(),
                                  SANDWITCH_BASE,
                                  sw_i as f64 * QUEUE_MOVE,
                                  ING_SIZE, QUEUE_ING_SPACING, -1.0);
        }
    }

    pub fn render_sandwitch<'a>(&self, cam: &mut Camera, ings: Iter<Ingredients>,
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
            Ingredients::Meat, Path::new("resources/textures/ingredient/meat.png"))?;
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
