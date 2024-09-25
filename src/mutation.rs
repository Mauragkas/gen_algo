use crate::frog::Frog;
use rand::Rng;

pub fn mutation_inplace<const D: usize>(frog: &mut Frog<D>, mutation_rate: f64) {
    let mut rng = rand::thread_rng();
    let mut mutation_sequence = [false; D];
    mutation_sequence.iter_mut().enumerate().for_each(|(i, mutate)| {
        *mutate = rng.gen::<f64>() < mutation_rate;
    });

    frog.chromosome.iter_mut().zip(mutation_sequence.iter()).for_each(|(gene, &mutate)| {
        if mutate {
            *gene = 1 - *gene;
        }
    });
}
