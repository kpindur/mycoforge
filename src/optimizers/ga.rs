use std::collections::HashMap;

use rand::Rng;

use crate::common::traits::{Crossoverer, Evaluator, Genotype, Individual, Initializer, Mutator, Optimizer, Selector};
use crate::operators::sampler::OperatorSampler;

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

#[macro_export]
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
