use nalgebra as na;
use rand::{Rng, RngCore};

pub struct Simulation {
    world: World,
}

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    food: Vec<Food>,
}

#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_movements();
        self.process_collisions(rng);
    }
    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.food {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    food.position = rng.r#gen();
                }
            }
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();

        let food = (0..40).map(|_| Food::random(rng)).collect();

        Self { animals, food }
    }
    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }
    pub fn food(&self) -> &[Food] {
        &self.food
    }
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.r#gen(),
            rotation: rng.r#gen(),
            speed: 0.002,
        }
    }
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.r#gen(),
        }
    }
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}
