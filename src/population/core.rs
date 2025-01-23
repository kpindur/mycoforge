use std::{error::Error, marker::PhantomData, time::Duration};

use crate::common::traits::{Genotype, Individual};

#[derive(Debug)]
pub enum PopulationError {
    PopulationFull(usize)
}

impl Error for PopulationError {}

impl std::fmt::Display for PopulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PopulationError::PopulationFull(size) => write!(f, "Population full: {}", size),
        }
    }
}

pub struct PopulationConfig {
    min_size: usize,
    max_size: usize,
    target_size: usize
}

impl Default for PopulationConfig {
    fn default() -> Self {
        return Self::new(0, 1000, 100);
    }
}

impl PopulationConfig {
    pub fn new(min_size: usize, max_size: usize, target_size: usize) -> Self {
        return Self { min_size, max_size, target_size };
    }
}

pub struct PopulationHistory {
    best_fitness: Vec<f64>,
    avg_fitness: Vec<f64>,
    population_sizes: Vec<usize>,
    timestamps: Vec<Duration>
}

impl Default for PopulationHistory {
    fn default() -> Self {
        return Self::new(
            Vec::new(), Vec::new(), 
            Vec::new(), Vec::new()
        );
    }
}

impl PopulationHistory {
    pub fn new(
        best_fitness: Vec<f64>, avg_fitness: Vec<f64>, 
        population_sizes: Vec<usize>, 
        timestamps: Vec<Duration>
    ) -> Self {
        return Self { best_fitness, avg_fitness, population_sizes, timestamps };
    }
}

pub struct Population<I, G>
where
    I: Individual<G>,
    G: Genotype,
{
    generation: usize,
    individuals: Vec<I>,
    config: PopulationConfig,
    history: PopulationHistory,
    _phantom: PhantomData<G>
}

impl<I, G> Default for Population<I, G>
where
    I: Individual<G>,
    G: Genotype
{
    fn default() -> Self {
        let config = PopulationConfig::default();
        let history = PopulationHistory::default();

        return Self::new(0, Vec::new(), config, history);
    }
}

impl<I, G> Population<I, G>
where
    I: Individual<G>,
    G: Genotype
{
    pub fn new(
        generation: usize, individuals: Vec<I>, 
        config: PopulationConfig, history: PopulationHistory
    ) -> Self {
        return Self { generation, individuals, config, history, _phantom: PhantomData };
    }

    pub fn add_individual(&mut self, individual: I) -> Result<(), PopulationError> {
        if self.individuals.len() >= self.config.max_size {
            return Err(PopulationError::PopulationFull(self.config.max_size));
        }
        self.individuals.push(individual);
        return Ok(());
    }

    pub fn next_generation(&mut self) {
        self.generation += 1;
        self.update_history();
    }

    fn update_history(&mut self) {
        let best = self.individuals.iter()
            .map(|i| i.phenotype())
            .fold(f64::NEG_INFINITY, f64::max);
        let avg = self.individuals.iter()
            .map(|i| i.phenotype())
            .sum::<f64>() / self.individuals.len() as f64;

        self.history.best_fitness.push(best);
        self.history.avg_fitness.push(avg);
        self.history.population_sizes.push(self.individuals.len());
    }
}
