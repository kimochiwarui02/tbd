use crate::*;

pub struct AnimalIndvidual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl ga::Individual for AnimalIndvidual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl AnimalIndvidual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.gluttony as f32,
            chromosome: animal.as_chromosome(),
        }
    }
    pub fn to_animal(self, rng: &mut dyn RngCore) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}
