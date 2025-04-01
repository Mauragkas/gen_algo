#![allow(unused)]
use num_format::{Locale, ToFormattedString};
use rayon::{prelude::*, ThreadPoolBuilder};
use select::*;
use std::{io::Write, time::Instant};

use crossover::*;
use frog::*;
use helper::*;
use mutation::*;

mod crossover;
mod frog;
mod helper;
mod mutation;
mod select;

fn main() {
    let start = Instant::now();
    const N: usize = 12 * 26; // Population size
    const D: usize = 64; // Chromosome size
    const MUTATION_RATE: f64 = 0.001;
    let generations: i32 = 100;
    // no need to run for 1_000_000 generations, but it's a good test
    // 100 generations is enough to see the algorithm working

    // Create a custom thread pool for Rayon
    let num_threads: usize = num_cpus::get(); // Get the number of logical cores
    let chunk_size: usize = (N / num_threads).max(1); // Calculate the chunk size for each thread pool
    let pool: rayon::ThreadPool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();

    pool.install(|| {
        let mut population = init_population::<N, D>();
        // let mut population = init_population_from_file("initial_population.csv");

        println!("Initial Population");
        println!(
            "Pop_size: {}, Chromosome_size: {}, Generations: {}, Mutation_rate: {}", N, D, generations.to_formatted_string(&Locale::en), MUTATION_RATE
        );
        let (max_fitness, frequency) = find_max_fitness_and_frequency(&population);
        println!(
            "Max Fitness: {} and percentage: {:.2}%",
            max_fitness,
            (frequency as f64 / N as f64) * 100.0
        );
        save_to_file(&population, "initial_population.csv");

        let start_of_generations = Instant::now();

        for generation in 0..generations {
            let throughput = generation as f64 / start_of_generations.elapsed().as_secs_f64();
            let eta_seconds = (generations - generation) as f64 / throughput;

            if generation % 500 == 0 { // for performance reasons, only print every 500 generations
                print!("\x1b[2K"); // Clear the current line
                print!(
                    "\x1b[1;32mGeneration: {}\x1b[0m, \x1b[1;34mThroughput: {:.2} gen/s\x1b[0m (ETA: {})\r",
                    generation,
                    throughput,
                    time_string(eta_seconds)
                );
                std::io::stdout().flush().unwrap();
            }

            let selected_population = selection_and_sort(&population);

            let mut children_population: [Frog<D>; N] = [Frog::new([0; D], 0); N];
            children_population
                .par_chunks_mut(chunk_size)
                .enumerate()
                .for_each(|(i, chunk)| {
                    // Ensure the chunk can hold enough children
                    for j in 0..chunk.len() / 2 {
                        let parent1 = &selected_population[i * chunk_size + j * 2];
                        let parent2 = &selected_population[i * chunk_size + j * 2 + 1];

                        let (mut child1, mut child2) = uniform_crossover(parent1, parent2);
                        mutation_inplace_small(&mut child1, MUTATION_RATE);
                        mutation_inplace_small(&mut child2, MUTATION_RATE);
                        child1.fitness_function();
                        child2.fitness_function();

                        // Store children in the current chunk
                        chunk[j * 2] = child1;
                        chunk[j * 2 + 1] = child2;
                    }
                });

            population = children_population;
        }

        println!();
        println!("{}", "-".repeat(50));
        println!("Final Population");
        let (max_fitness, frequency) = find_max_fitness_and_frequency(&population);
        save_to_file(&population, "final_population.csv");
        println!(
            "Max Fitness: {} and percentage: {:.2}%",
            max_fitness,
            (frequency as f64 / N as f64) * 100.0
        );
        println!("Time taken: {:.4?}", start.elapsed());
    });
}
