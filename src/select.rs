#![allow(unused)]
use crate::frog::Frog;
use crate::NewTrait;
use rand::Rng;

pub fn selection_and_sort<const N: usize, const D: usize>(
    population: &[Frog<D>; N],
) -> [Frog<D>; N] {
    let total_fitness: u32 = population.iter().map(|f| f.fitness).sum();
    let mut rng = rand::thread_rng();

    let cumulative_fitness: [u32; N] = {
        let mut cumulative = 0;
        let mut cum_fit = [0; N];
        for (i, frog) in population.iter().enumerate() {
            cumulative += frog.fitness;
            cum_fit[i] = cumulative;
        }
        cum_fit
    };

    let mut selected_population: [Frog<D>; N] = [Frog::new([0; D], 0); N];
    for selected in selected_population.iter_mut() {
        let random_number = rng.gen_range(0..total_fitness);
        let index = cumulative_fitness
            .iter()
            .position(|&cum_fit| cum_fit > random_number)
            .unwrap();
        *selected = population[index].clone();
    }

    selected_population.sort_by(|a, b| b.fitness.cmp(&a.fitness));

    selected_population
}
