use crate::frog::Frog;
use rand::{thread_rng, Rng};

pub fn mutation_inplace<const D: usize>(frog: &mut Frog<D>, mutation_rate: f64) {
    let mut rng = thread_rng();

    // Process 64 bits at a time using u64
    const CHUNK_SIZE: usize = 64;
    let chunks = D / CHUNK_SIZE;
    let remainder = D % CHUNK_SIZE;

    // Generate random u64 and compare with mutation threshold
    let threshold = (mutation_rate * (u64::MAX as f64)) as u64;

    for chunk_idx in 0..chunks {
        let random_bits = rng.gen::<u64>();
        let mutations = (0..CHUNK_SIZE)
            .filter(|&i| ((random_bits >> i) & 1) == 1)
            .filter(|_| rng.gen::<u64>() < threshold);

        for pos in mutations {
            let idx = chunk_idx * CHUNK_SIZE + pos;
            frog.chromosome[idx] = 1 - frog.chromosome[idx];
        }
    }

    // Handle remaining bits
    if remainder > 0 {
        let random_bits = rng.gen::<u64>() & ((1 << remainder) - 1);
        let start_idx = chunks * CHUNK_SIZE;

        let mutations = (0..remainder)
            .filter(|&i| ((random_bits >> i) & 1) == 1)
            .filter(|_| rng.gen::<u64>() < threshold);

        for pos in mutations {
            let idx = start_idx + pos;
            frog.chromosome[idx] = 1 - frog.chromosome[idx];
        }
    }
}

// If your chromosome size is fixed and small, this version might be even faster
pub fn mutation_inplace_small<const D: usize>(frog: &mut Frog<D>, mutation_rate: f64) {
    let mut rng = thread_rng();
    let threshold = (mutation_rate * (u64::MAX as f64)) as u64;

    // Generate a single random number for the entire chromosome if D <= 64
    if D <= 64 {
        let random_bits = rng.gen::<u64>() & ((1u64 << D) - 1);
        let mutation_mask = rng.gen::<u64>() & ((1u64 << D) - 1);

        let final_mask = random_bits & mutation_mask;
        for i in 0..D {
            if ((final_mask >> i) & 1) == 1 && rng.gen::<u64>() < threshold {
                frog.chromosome[i] = 1 - frog.chromosome[i];
            }
        }
    }
}
