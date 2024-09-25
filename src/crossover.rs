use crate::{frog::Frog, NewTrait};
use rand::Rng;

pub fn crossover<const D: usize>(frog1: &Frog<D>, frog2: &Frog<D>) -> (Frog<D>, Frog<D>) {
    let mut rng = rand::thread_rng();
    let crossover_point = rng.gen_range(0..D);

    let mut child1 = Frog::new([0; D], 0);
    let mut child2 = Frog::new([0; D], 0);

    for i in 0..D {
        if i < crossover_point {
            child1.chromosome[i] = frog1.chromosome[i];
            child2.chromosome[i] = frog2.chromosome[i];
        } else {
            child1.chromosome[i] = frog2.chromosome[i];
            child2.chromosome[i] = frog1.chromosome[i];
        }
    }

    (child1, child2)
}

pub fn uniform_crossover<const D: usize>(frog1: &Frog<D>, frog2: &Frog<D>) -> (Frog<D>, Frog<D>) {
    let mut rng = rand::thread_rng();
    let mut random_bits = vec![false; D];

    // Generate all random bits in one go
    random_bits
        .iter_mut()
        .map(|bit| *bit = rng.gen_bool(0.5))
        .count();

    let mut child1 = Frog::new([0; D], 0);
    let mut child2 = Frog::new([0; D], 0);

    random_bits
        .iter()
        .enumerate()
        .map(|(i, &bit)| {
            if bit {
                child1.chromosome[i] = frog1.chromosome[i];
                child2.chromosome[i] = frog2.chromosome[i];
            } else {
                child1.chromosome[i] = frog2.chromosome[i];
                child2.chromosome[i] = frog1.chromosome[i];
            }
        })
        .count();

    (child1, child2)
}
