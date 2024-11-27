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

#[derive(Debug)]
pub enum BuilderError {
    IncorrectWeight,
    KeyExists,
    OperatorsIsEmpty,
    WrongWeightSum
}

impl std::error::Error for BuilderError {}
impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncorrectWeight => write!(f, "IncorrectWeight"),
            Self::KeyExists => write!(f, "KeyExists"),
            Self::OperatorsIsEmpty => write!(f, "OperatorsIsEmpty"),
            Self::WrongWeightSum => write!(f, "WrongWeightSum"),
        }
    }
}

pub struct OperatorsBuilder {
    operators: HashMap<String, Functor>,
    weights_sum: f64,
}

impl OperatorsBuilder {
    pub fn new(operators: HashMap<String, Functor>, weights_sum: f64) -> Self {
        return Self { operators, weights_sum };
    }

    pub fn add_operator(mut self, name: &str, func: VectorFunction, arity: usize, weight: f64) 
        -> Result<Self, BuilderError> {
            if weight <= 0.0 || weight > 1.0 { return Err(BuilderError::IncorrectWeight); }
            if self.operators.contains_key(name) { return Err(BuilderError::KeyExists); }
            
            self.operators.insert(name.to_string(), Functor::new(func, arity, weight));
            self.weights_sum += weight;

            return Ok(self);
    }

    pub fn build(self) -> Result<Operators, BuilderError> {
        if self.operators.is_empty() { return Err(BuilderError::OperatorsIsEmpty); }
        if (self.weights_sum - 1.0).abs() > 1e-10 { return Err(BuilderError::WrongWeightSum); }
        
        let capacity = self.operators.len();
        let (mut ops, mut arity, mut weights) = 
            (Vec::with_capacity(capacity), Vec::with_capacity(capacity), Vec::with_capacity(capacity));

        for (name, func) in &self.operators {
            ops.push(name.clone());
            arity.push(func.arity());
            weights.push(func.weight());
        }

        return Ok(Operators::new(self.operators, OperatorSampler::new(ops, arity, weights)));
    }
}

impl Default for OperatorsBuilder {
    fn default() -> Self { return Self { operators: HashMap::new(), weights_sum: 0.0 }; }
}

