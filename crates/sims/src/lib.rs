mod animal;
mod animal_indvidual;
mod brain;
mod eye;
mod food;
mod world;
pub use self::{animal::*, animal_indvidual::*, brain::*, eye::*, food::*, world::*};
use lib_gen_algos::{
    self as ga, CrossoverMethod, GaussianMutation, RouletteWheelSelection, UniformCrossover,
};
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_2;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.002;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const GENERATION_LENGTH: usize = 2500;

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgo<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);

        let ga = ga::GeneticAlgo::new(
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.01, 0.3),
        );

        Self { world, ga, age: 0 }
    }
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<ga::Statistics> {
        self.process_movements();
        self.process_brains();
        self.process_collisions(rng);

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        self.age = 0;
        let current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndvidual::from_animal)
            .collect();

        let (evolved_population, stats) = self.ga.evolve(rng, &current_population);

        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.to_animal(rng))
            .collect();

        for food in &mut self.world.food {
            food.position = rng.r#gen();
        }
        stats
    }
    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.food {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    animal.gluttony += 1;
                    food.position = rng.r#gen();
                }
            }
        }
    }
    pub fn train(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        loop {
            if let Some(summary) = self.step(rng) {
                return summary;
            }
        }
    }
    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.food);
            let response = animal.brain.nn.propagate(vision);
            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);
            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
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
