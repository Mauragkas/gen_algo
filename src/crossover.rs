use crate::{frog::Frog, FrogTrait};
use rand::Rng;

/// Single-point crossover with improved performance
#[inline]
pub fn crossover<const D: usize>(frog1: &Frog<D>, frog2: &Frog<D>) -> (Frog<D>, Frog<D>) {
    let mut rng = rand::thread_rng();
    let crossover_point = rng.gen_range(0..D);

    // Create children with zeroed chromosomes
    let mut child1 = Frog::new([0; D], 0);
    let mut child2 = Frog::new([0; D], 0);

    // Copy first part directly using slice operations
    child1.chromosome[..crossover_point].copy_from_slice(&frog1.chromosome[..crossover_point]);
    child2.chromosome[..crossover_point].copy_from_slice(&frog2.chromosome[..crossover_point]);

    // Copy second part directly using slice operations
    child1.chromosome[crossover_point..].copy_from_slice(&frog2.chromosome[crossover_point..]);
    child2.chromosome[crossover_point..].copy_from_slice(&frog1.chromosome[crossover_point..]);

    (child1, child2)
}

/// Uniform crossover with improved performance
#[inline]
pub fn uniform_crossover<const D: usize>(frog1: &Frog<D>, frog2: &Frog<D>) -> (Frog<D>, Frog<D>) {
    let mut rng = rand::thread_rng();
    let mut child1 = Frog::new([0; D], 0);
    let mut child2 = Frog::new([0; D], 0);

    // Process chunks of 64 bits at a time for better performance
    for chunk in (0..D).step_by(64) {
        let random_bits = rng.gen::<u64>();
        let end = (chunk + 64).min(D);

        for (i, bit_pos) in (chunk..end).enumerate() {
            if (random_bits >> i) & 1 == 1 {
                child1.chromosome[bit_pos] = frog1.chromosome[bit_pos];
                child2.chromosome[bit_pos] = frog2.chromosome[bit_pos];
            } else {
                child1.chromosome[bit_pos] = frog2.chromosome[bit_pos];
                child2.chromosome[bit_pos] = frog1.chromosome[bit_pos];
            }
        }
    }

    (child1, child2)
}
