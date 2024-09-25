#![allow(unused)]
use rand::Rng;
use rayon::{prelude::*, ThreadPoolBuilder};
use select::*;
use std::io::Write;

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
    let start = std::time::Instant::now();
    const N: usize = 300; // Population size
    const D: usize = 64; // Chromosome size
    const CHUNK_SIZE: usize = 4;
    let generations: i32 = 1_000_000;
    let mutation_rate: f64 = 0.001;

    // Create a custom thread pool for Rayon
    let num_threads: usize = num_cpus::get(); // Get the number of logical cores
    let pool: rayon::ThreadPool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();

    pool.install(|| {
        let mut population = init_population::<N, D>();
        // let mut population = init_population_from_file("initial_population.csv");

        println!("Initial Population");
        println!(
            "Pop_size: {}, Chromosome_size: {}, Generations: {}, Mutation_rate: {}", N, D, generations, mutation_rate
        );
        let (mut max_fitness, mut frequency) = find_max_fitness_and_frequency(&population);
        println!(
            "Max Fitness: {} and percentage: {:.2}%",
            max_fitness,
            (frequency as f64 / N as f64) * 100.0
        );
        // save_to_file(&population, "initial_population.csv");

        let start_of_generations = std::time::Instant::now();
        let mut total_time = 0.0;

        for generation in 0..generations {
            let throughput = generation as f64 / start_of_generations.elapsed().as_secs_f64();
            let eta_seconds = (generations - generation) as f64 / throughput;

            print!("\x1b[2K"); // Clear the current line
            print!(
                "\x1b[1;32mGeneration: {}\x1b[0m, \x1b[1;34mThroughput: {:.2} gen/s\x1b[0m (ETA: {})\r",
                generation, throughput, time_string(eta_seconds)
            );
            std::io::stdout().flush().unwrap();

            let selected_population = selection_and_sort(&population);

            let mut children_population: [Frog<D>; N] = [Frog::new([0; D], 0); N];
            children_population
                .par_chunks_mut(CHUNK_SIZE)
                .enumerate()
                .for_each(|(i, chunk)| {
                    // Ensure the chunk can hold enough children
                    for j in 0..chunk.len() / 2 {
                        let parent1 = &selected_population[i * CHUNK_SIZE + j * 2];
                        let parent2 = &selected_population[i * CHUNK_SIZE + j * 2 + 1];

                        let (mut child1, mut child2) = uniform_crossover(parent1, parent2);
                        mutation_inplace(&mut child1, mutation_rate);
                        mutation_inplace(&mut child2, mutation_rate);
                        child1.fitness_function();
                        child2.fitness_function();

                        // Store children in the current chunk
                        chunk[j * 2] = child1;
                        chunk[j * 2 + 1] = child2;
                    }
                });

            let start = std::time::Instant::now();

            population = children_population;
        }

        println!();
        println!("{}", "-".repeat(50));
        println!("Final Population");
        let (max_fitness, frequency) = find_max_fitness_and_frequency(&population);
        println!(
            "Max Fitness: {} and percentage: {:.2}%",
            max_fitness,
            (frequency as f64 / N as f64) * 100.0
        );
        println!("Time taken: {:.4?}", start.elapsed());
        // save_to_file(&population, "final_population.csv");
    });
}
