use rand::{seq::{IndexedRandom}, RngCore};

pub struct GeneticAlgo;

impl GeneticAlgo {
    pub fn evolve(&self, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());
        (0..population.len()).map(|_| todo!()).collect()
    }
}

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

