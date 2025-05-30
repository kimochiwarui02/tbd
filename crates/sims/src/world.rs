use crate::*;

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) food: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..10).map(|_| Animal::random(rng)).collect();

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
