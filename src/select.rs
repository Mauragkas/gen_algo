#![allow(unused)]
use crate::frog::Frog;
use crate::FrogTrait;
use rand::Rng;

/// Optimized selection and sorting function using binary search and better memory handling
#[inline]
pub fn selection_and_sort<const N: usize, const D: usize>(
    population: &[Frog<D>; N],
) -> [Frog<D>; N] {
    // Calculate total fitness and cumulative probabilities in one pass
    let (total_fitness, cumulative_fitness) = {
        let mut cumulative = 0u32;
        let mut cum_fit = [0u32; N];

        for (i, frog) in population.iter().enumerate() {
            cumulative += frog.fitness;
            cum_fit[i] = cumulative;
        }
        (cumulative, cum_fit)
    };

    let mut selected_population = [Frog::new([0; D], 0); N];
    let mut rng = rand::thread_rng();

    // Binary search implementation for finding position
    #[inline]
    fn binary_search(arr: &[u32], target: u32) -> usize {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    // Batch random number generation for better performance
    const BATCH_SIZE: usize = 32;
    let mut i = 0;
    while i < N {
        let batch_end = (i + BATCH_SIZE).min(N);
        let batch_size = batch_end - i;

        // Generate batch of random numbers
        let random_numbers: Vec<u32> = (0..batch_size)
            .map(|_| rng.gen_range(0..total_fitness))
            .collect();

        // Process batch
        for (j, &random_number) in random_numbers.iter().enumerate() {
            let index = binary_search(&cumulative_fitness, random_number);
            selected_population[i + j] = population[index].clone();
        }

        i = batch_end;
    }

    // Sort in descending order of fitness
    selected_population.sort_unstable_by(|a, b| b.fitness.cmp(&a.fitness));

    selected_population
}
