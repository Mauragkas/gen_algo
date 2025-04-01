# Genetic Algorithm Experiment

This is an experimental implementation of a genetic algorithm in Rust, created for learning and performance optimization purposes. The project demonstrates various genetic algorithm concepts while focusing on performance optimizations using parallel processing and efficient data structures.

## Overview

This genetic algorithm implementation includes:

- Population initialization with random chromosomes
- Fitness calculation
- Selection (using binary search and batch processing)
- Crossover (single-point and uniform)
- Mutation
- Parallel processing using Rayon
- Performance optimizations for different chromosome sizes

## Structure

The project is organized into several modules:

- `main.rs`: Main program logic and configuration
- `frog.rs`: Definition of the individual (Frog) and its traits
- `crossover.rs`: Crossover operations implementation
- `mutation.rs`: Mutation operations implementation
- `select.rs`: Selection algorithm implementation
- `helper.rs`: Utility functions for population management and I/O

## Features

- Configurable population size and chromosome length
- Adjustable mutation rate
- Progress tracking with ETA and throughput calculation
- CSV file I/O for population state
- Optimized performance for both small and large chromosomes
- Multi-threaded processing using Rayon

## Running the Project

1. Make sure you have Rust installed on your system.

2. Clone the repository and navigate to the project directory.

3. Build and run the project:
```bash
cargo run --release
```

Note: The `--release` flag is important for performance as it enables optimizations.

## Configuration

You can modify these constants in `main.rs` to experiment with different parameters:

```rust
const N: usize = 12 * 26;      // Population size
const D: usize = 64;           // Chromosome size
const MUTATION_RATE: f64 = 0.001;
let generations: i32 = 100;    // Number of generations
```

## Output

The program outputs:
- Initial population statistics
- Generation progress with throughput and ETA
- Final population statistics
- Total execution time

## File I/O

The program creates two CSV files:
- `initial_population.csv`: Initial population state
- `final_population.csv`: Final population state

## Note

This is an experimental project created for learning purposes and performance optimization experiments. It may not be suitable for production use without further modifications and testing.

## Performance Considerations

- The implementation includes optimizations for different chromosome sizes
- Parallel processing is used where beneficial
- Memory access patterns are optimized for better cache utilization
- Batch processing is implemented for certain operations

## License

Feel free to use and modify this code for your own learning and experimentation purposes.
