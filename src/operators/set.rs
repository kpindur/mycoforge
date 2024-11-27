type VectorFunction = fn(&[&[f64]]) -> Vec<f64>;

pub trait OperatorSet {
    fn get_operator(&self, name: &str) -> Option<&Functor>;
    fn sample<R: Rng>(&self, rng: &mut R) -> (String, usize);
}

pub struct Operators {
    operators: HashMap<String, Functor>,
    sampler: OperatorSampler,
}

impl Operators {
    pub fn new(operators: HashMap<String, Functor>, sampler: OperatorSampler) -> Self { 
        return Self { operators, sampler }; 
    }

    pub fn operators(&self) -> &HashMap<String, Functor> { return &self.operators; }
    pub fn sampler(&self) -> &OperatorSampler { return &self.sampler; }

    pub fn create_map(&self) -> HashMap<String, (usize, VectorFunction)> {
        let mut map = HashMap::new();
        for (key, value) in &self.operators {
            map.insert(key.clone(), (value.arity(), *value.func()));
        }
        return map;
    }
}

impl OperatorSet for Operators {
    fn get_operator(&self, name: &str) -> Option<&Functor> { return self.operators.get(name); }
    fn sample<R: Rng>(&self, rng: &mut R) -> (String, usize) { return self.sampler.sample(rng); }
}


#[derive(Clone)]
pub struct Functor {
    func: VectorFunction,
    arity: usize,
    weight: f64
}

impl Functor {
    pub fn new(func: VectorFunction, arity: usize, weight: f64) -> Self { return Self { func, arity, weight }; }

    pub fn arity(&self) -> usize { return self.arity; }
    pub fn weight(&self) -> f64 { return self.weight; }
    pub fn func(&self) -> &VectorFunction { return &self.func; }
}

