use std::collections::HashMap;

use rand::Rng;

use crate::common::traits::{Crossoverer, Evaluator, Genotype, Individual, Initializer, Mutator, Optimizer, Selector};
use crate::tree::core::sampler::OperatorSampler;

pub trait EAComponents<G: Genotype> {
    type I: Individual<G>;
    type Init: Initializer<G>;
    type Mut: Mutator<G>;
    type Eval: Evaluator<G>;
    type Cross: Crossoverer<G>;
    type Sel: Selector<G, I = Self::I>;
}

pub struct EA<C: EAComponents<G>, G: Genotype>
{
    initializer:    C::Init,
    mutator:        C::Mut,
    crossoverer:    C::Cross,
    evaluator:      C::Eval,
    selector:       C::Sel,
    sampler:        OperatorSampler,
    map:            HashMap<String, (usize, fn(&[&[f64]]) -> Vec<f64>)>
}

impl<C, G> EA<C, G> 
where
    G: Genotype,
    C: EAComponents<G>,
{
    pub fn new(initializer: C::Init, 
        mutator: C::Mut, crossoverer: C::Cross, evaluator: C::Eval, selector: C::Sel, 
        sampler: OperatorSampler, map: HashMap<String, (usize, fn(&[&[f64]]) -> Vec<f64>)>) -> Self 
    {
        return Self { initializer, mutator, crossoverer, evaluator, selector, sampler, map };
    }

    pub fn evaluator(&self) -> &C::Eval { return &self.evaluator; }
    pub fn map(&self) -> &HashMap<String, (usize, fn(&[&[f64]]) -> Vec<f64>)> { return &self.map; }
}

impl<C, G> Optimizer<G> for EA<C, G> 
where
    G: Genotype,
    C: EAComponents<G>,
{
    type I = C::I;
    fn init_population<R: Rng>(&self, rng: &mut R, population_size: usize) -> Vec<G> {
        return (0..population_size).map(|_| self.initializer.initialize(rng, &self.sampler)).collect();
    }

    fn optimize<R: Rng>(&self, rng: &mut R, population: &[Self::I]) -> Vec<G> {
        let mut offspring: Vec<G> = Vec::with_capacity(population.len());

        while offspring.len() < population.len() {
            let parent1 = self.selector.select(rng, population);
            let parent2 = self.selector.select(rng, population);

            let children = self.crossoverer.variate(rng, &parent1, &parent2, &self.sampler);

            for child in children {
                offspring.push(
                    self.mutator.variate(rng, &child, &self.sampler)
                );
                if offspring.len() >= population.len() { break; }
            }

        }
        return offspring;
    }
}

#[derive(Debug)]
pub enum BuilderError {
   InitializerMissing,
   MutatorMissing,
   CrossovererMissing,
   EvaluatorMissing,
   SelectorMissing,
   SamplerMissing,
   MapMissing,
}

pub struct EABuilder<C: EAComponents<G>, G: Genotype> {
    initializer:    Option<C::Init>,
    mutator:        Option<C::Mut>,
    crossoverer:    Option<C::Cross>,
    evaluator:      Option<C::Eval>,
    selector:       Option<C::Sel>,
    sampler:        Option<OperatorSampler>,
    map:            Option<HashMap<String, (usize, fn(&[&[f64]]) -> Vec<f64>)>>
}

impl<C, G> EABuilder<C, G> 
where
    G: Genotype,
    C: EAComponents<G>,
{
    pub fn new() -> Self {
        return Self { initializer: None, mutator: None, crossoverer: None, evaluator: None, selector: None, sampler: None, map: None };
    }

    pub fn build(self) -> Result<EA<C, G>, BuilderError> {
       return Ok(EA {
           initializer: self.initializer.ok_or(BuilderError::InitializerMissing)?,
           mutator:     self.mutator.ok_or(BuilderError::MutatorMissing)?,
           crossoverer: self.crossoverer.ok_or(BuilderError::CrossovererMissing)?,
           evaluator: self.evaluator.ok_or(BuilderError::EvaluatorMissing)?,
           selector:    self.selector.ok_or(BuilderError::SelectorMissing)?,
           sampler:     self.sampler.ok_or(BuilderError::SamplerMissing)?,
           map:         self.map.ok_or(BuilderError::MapMissing)?,
       })
   }

    pub fn set_initializer(mut self, initializer: C::Init) -> Self { 
       self.initializer = Some(initializer);
       return self;
   } 

   pub fn set_mutator(mut self, mutator: C::Mut) -> Self { 
       self.mutator = Some(mutator);
       return self;
   }

   pub fn set_crossoverer(mut self, crossoverer: C::Cross) -> Self { 
       self.crossoverer = Some(crossoverer);
       return self;
   }

   pub fn set_evaluator(mut self, evaluator: C::Eval) -> Self {
       self.evaluator = Some(evaluator);
       return self;
   }

   pub fn set_selector(mut self, selector: C::Sel) -> Self { 
       self.selector = Some(selector);
       return self;
   }

   pub fn set_sampler(mut self, sampler: OperatorSampler) -> Self { 
       self.sampler = Some(sampler);
       return self;
   }

   pub fn set_map(mut self, map: HashMap<String, (usize, fn(&[&[f64]]) -> Vec<f64>)>) -> Self { 
       self.map = Some(map);
       return self;
   }

}

macro_rules! ea_components {
    (
        genotype: $genotype:ty,
        individual: $individual:ty,
        components: {
            init: $init_type:ty,
            mutation: $mut_type:ty,
            crossover: $cross_type:ty,
            evaluation: $eval_type:ty,
            selection: $sel_type:ty
        },
        operators: {
            $(
                $op_name:literal => ($func:ident, $arity:literal, $weight:literal)
            ),*$(,)?
        },
        config: {
            init: $init:expr,
            mutation: $mutation:expr,
            crossover: $crossover:expr,
            evaluation: $evaluation:expr,
            selection: $selection:expr
       }
    ) => {
        {
            struct Components;
            impl EAComponents<$genotype> for Components {
                type I = $individual;
                type Init = $init_type;
                type Mut = $mut_type;
                type Cross = $cross_type;
                type Eval = $eval_type;
                type Sel = $sel_type;
            }

            let operators: Vec<String> = vec![$($op_name.to_string()),*];
            let functions: Vec<fn(&[&[f64]]) -> Vec<f64>> = vec![$($func),*];
            let arity = vec![$($arity),*];
            let weights = vec![$($weight), *];

            let sampler = OperatorSampler::new(operators.clone(), arity.clone(), weights);

            let map: HashMap<String, (usize, fn(&[&[f64]]) -> Vec<f64>)> = operators.into_iter()
                .zip(arity.iter().zip(functions.iter()))
                .map(|(op, (&ar, &func))| (op, (ar, func))).collect();

            EABuilder::<Components, $genotype>::new()
                .set_initializer($init)
                .set_mutator($mutation)
                .set_crossoverer($crossover)
                .set_evaluator($evaluation)
                .set_selector($selection)
                .set_sampler(sampler)
                .set_map(map)
                .build()
                .expect("EA should be properly constructed!")
        }
    };
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tree::{core::{individual::TreeIndividual, sampler::OperatorSampler, tree::TreeGenotype}, fitness::evaluate::MeanSquared, operators::{crossover::SubtreeCrossover, init::Grow, mutation::SubtreeMutation, select::TournamentSelection}};

    fn add(args: &[&[f64]]) -> Vec<f64> {
        if args.len() != 2 || args[0].is_empty() || args[1].is_empty() { return Vec::new(); }
        return args[0].iter().zip(args[1].iter())
            .map(|(&a, &b)| a + b)
            .collect::<Vec<f64>>();
    }
    fn sub(args: &[&[f64]]) -> Vec<f64> {
        if args.len() != 2 || args[0].is_empty() || args[1].is_empty() { return Vec::new(); }
        return args[0].iter().zip(args[1].iter())
            .map(|(&a, &b)| a - b)
            .collect::<Vec<f64>>();
    }
    fn mul(args: &[&[f64]]) -> Vec<f64> {
        if args.len() != 2 || args[0].is_empty() || args[1].is_empty() { return Vec::new(); }
        return args[0].iter().zip(args[1].iter())
            .map(|(&a, &b)| a * b)
            .collect::<Vec<f64>>();
    }
    fn sin(args:&[&[f64]]) -> Vec<f64> {
        if args.len() != 1 || args[0].is_empty() { return Vec::new(); }
        return args[0].iter()
            .map(|&a| a.sin())
            .collect::<Vec<f64>>();
    }
    fn x(args:&[&[f64]]) -> Vec<f64> {
        return args[0].to_vec();
    }
        
    #[test]
    fn test_builder_works() {
        struct Components;
        impl EAComponents<TreeGenotype> for Components {
            type I = TreeIndividual<TreeGenotype>;
            type Init = Grow;
            type Mut = SubtreeMutation;
            type Cross = SubtreeCrossover;
            type Eval = MeanSquared;
            type Sel = TournamentSelection;
        }

        let functions: Vec<fn(&[&[f64]])-> Vec<f64>> = vec![ add, sub, mul, sin, x ];
        let operators: Vec<String> = ["+", "-", "*","sin", "x"].iter().map(|&w| w.to_string()).collect();
        let arity = vec![2, 2, 2, 1, 0];
        let weights = vec![1.0 / 5.0; 5];


        let init_scheme = Grow::new(2, 4);
        let mutation_scheme = SubtreeMutation::new(1.0);
        let crossover_scheme = SubtreeCrossover::new(1.0);
        let evaluation_scheme = MeanSquared::new();
        let selection_scheme = TournamentSelection::new(5);

        let sampler = OperatorSampler::new(operators.clone(), arity.clone(), weights);

        let map: HashMap<String, (usize, fn(&[&[f64]])-> Vec<f64>)> = operators.into_iter()
            .zip(arity.iter().zip(functions.iter())).map(|(la, (&ar, &op))| (la, (ar, op))).collect();

        let _ = EABuilder::<Components, TreeGenotype>::new()
            .set_initializer(init_scheme)
            .set_mutator(mutation_scheme)
            .set_crossoverer(crossover_scheme)
            .set_evaluator(evaluation_scheme)
            .set_selector(selection_scheme)
            .set_sampler(sampler)
            .set_map(map)
            .build()
            .expect("EA should be properly constructed");
    }

    #[test]
    fn test_macro_works() {
        let _ = ea_components! {
            genotype: TreeGenotype,
            individual: TreeIndividual<TreeGenotype>,
            components: {
                init: Grow,
                mutation: SubtreeMutation,
                crossover: SubtreeCrossover,
                evaluation: MeanSquared,
                selection: TournamentSelection
            },
            operators: {
                "+" => (add, 2, 0.2),
                "-" => (sub, 2, 0.2),
                "*" => (mul, 2, 0.2),
                "sin" => (sin, 1, 0.2),
                "x" => (x, 0, 0.2)
            },
            config: {
                init: Grow::new(2, 4),
                mutation: SubtreeMutation::new(0.1),
                crossover: SubtreeCrossover::new(0.9),
                evaluation: MeanSquared::new(),
                selection: TournamentSelection::new(5)
            }
        };
    }
}
