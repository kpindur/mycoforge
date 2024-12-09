use mycoforge::tree::core::tree::TreeGenotype;
use rstest::{fixture, rstest};

use rand::rngs::StdRng;
use rand::{thread_rng, SeedableRng};

use mycoforge::common::traits::{Initializer, Selector};

use mycoforge::operators::sampler::OperatorSampler;
use mycoforge::tree::core::individual::TreeIndividual;

use mycoforge::tree::operators::init::Grow;
use mycoforge::tree::operators::select::TournamentSelection;

#[fixture]
fn sample_sampler() -> OperatorSampler {
    let operators: Vec<String> = ["+", "-", "sin", "x", "y", "z"].iter().map(|&w| w.to_string()).collect();
    let arity = vec![2, 2, 1, 0, 0, 0];
    let weights = vec![1.0 / 6.0; 6];

    let sampler = OperatorSampler::new(operators, arity, weights);
    
    return sampler;
}

#[fixture]
fn sample_population(sample_sampler: OperatorSampler) -> Vec<TreeIndividual<TreeGenotype>> {
    let mut rng = thread_rng();
    let init_scheme = Grow::new(2, 4);
    let population = (0..10)
        .map(|i| TreeIndividual::new(init_scheme.initialize(&mut rng, &sample_sampler), i as f64))
        .collect();
    return population;
}

#[rstest]
#[should_panic]
fn test_tournament_too_large(sample_population: Vec<TreeIndividual<TreeGenotype>>) {
    let mut rng = thread_rng();

    let selection = TournamentSelection::new(11);
    let _ = selection.select(&mut rng, &sample_population);
}

#[rstest]
#[case(1)]
#[case(5)]
#[case(10)]
fn test_tournament_selection(#[case] size: usize, sample_population: Vec<TreeIndividual<TreeGenotype>>) {
    let mut rng = StdRng::seed_from_u64(42);
    
    let selection = TournamentSelection::new(size);
    let chosen = selection.select(&mut rng, &sample_population);
    println!("{}", chosen);
}
