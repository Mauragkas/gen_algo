#![allow(unused)]
use crate::frog::Frog;
use crate::NewTrait;
use csv::Writer;
use rand::Rng;
use rayon::prelude::*;

pub fn find_max_fitness_and_frequency<const N: usize, const D: usize>(
    population: &[Frog<D>; N],
) -> (u32, u32) {
    use rayon::prelude::*;

    let (max_fitness, frequency) = population
        .par_iter()
        .fold(
            || (0, 0),
            |(max_fitness, frequency), frog| {
                if frog.fitness > max_fitness {
                    (frog.fitness, 1)
                } else if frog.fitness == max_fitness {
                    (max_fitness, frequency + 1)
                } else {
                    (max_fitness, frequency)
                }
            },
        )
        .reduce(
            || (0, 0),
            |(max_fitness1, frequency1), (max_fitness2, frequency2)| {
                if max_fitness1 > max_fitness2 {
                    (max_fitness1, frequency1)
                } else if max_fitness1 < max_fitness2 {
                    (max_fitness2, frequency2)
                } else {
                    (max_fitness1, frequency1 + frequency2)
                }
            },
        );

    (max_fitness, frequency)
}

pub fn give_chromosome<const D: usize>() -> ([u8; D], u32) {
    let mut chromosome = [0; D];
    let mut fitness = 0;
    for i in 0..D {
        chromosome[i] = rand::random::<u8>() % 2;
        fitness += chromosome[i] as u32;
    }
    (chromosome, fitness)
}

pub fn init_population<const N: usize, const D: usize>() -> [Frog<D>; N] {
    let mut population: [Frog<D>; N] = [Frog::new([0; D], 0); N]; // Initialize with zeroes
    population.par_iter_mut().for_each(|frog| {
        let (chromosome, fitness) = give_chromosome::<D>();
        frog.set_chromosome(chromosome);
        frog.fitness = fitness;
    });
    population
}

pub fn init_population_from_file<const N: usize, const D: usize>(filename: &str) -> [Frog<D>; N] {
    let mut rdr = csv::Reader::from_path(filename).expect("Unable to read file");
    let mut population: [Frog<D>; N] = [Frog::new([0; D], 0); N]; // Initialize with zeroes
    for (i, result) in rdr.deserialize().enumerate() {
        let (chromosome, fitness): (String, u32) = result.expect("Unable to deserialize");
        let chromosome: [u8; D] = chromosome
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
            .try_into()
            .expect("Unable to convert into array");
        population[i] = Frog::new(chromosome, fitness);
    }
    population
}

pub fn save_to_file<const N: usize, const D: usize>(population: &[Frog<D>; N], filename: &str) {
    let mut wtr = csv::Writer::from_path(filename).expect("Unable to create file");
    for frog in population.iter() {
        // get the chromosome as a string
        let chromosome: String = frog
            .chromosome
            .iter()
            .map(|&gene| gene.to_string())
            .collect::<Vec<String>>()
            .join("");
        wtr.serialize((chromosome, frog.fitness))
            .expect("Unable to serialize");
    }
    wtr.flush().expect("Unable to flush");
}

pub fn time_string(seconds: f64) -> String {
    let days = (seconds / 86400.0).floor();
    let hours = (seconds / 3600.0).floor();
    let minutes = ((seconds % 3600.0) / 60.0).floor();
    let seconds = seconds % 60.0;
    format!("{:.0}D {:.0}h {:.0}m {:.0}s", days, hours, minutes, seconds)
}
