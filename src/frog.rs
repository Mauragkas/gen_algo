use rand::Rng;
use rayon::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, Copy)]
pub(crate) struct Frog<const D: usize> {
    pub chromosome: [u8; D],
    pub fitness: u32,
}

pub trait NewTrait<const D: usize> {
    fn new(chromosome: [u8; D], fitness: u32) -> Frog<D>;
    fn set_chromosome(&mut self, chromosome: [u8; D]);
    fn get_chromosome(&self) -> &[u8; D];
    fn fitness_function(&mut self);
}

impl<const D: usize> NewTrait<D> for Frog<D> {
    fn new(chromosome: [u8; D], fitness: u32) -> Frog<D> {
        Frog {
            chromosome,
            fitness,
        }
    }

    fn set_chromosome(&mut self, chromosome: [u8; D]) {
        self.chromosome = chromosome;
    }

    fn get_chromosome(&self) -> &[u8; D] {
        &self.chromosome
    }

    fn fitness_function(&mut self) {
        self.fitness = self.chromosome.iter().map(|&gene| gene as u32).sum();
    }
}
