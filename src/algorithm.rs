use randomx::{RandomXCache, RandomXFlag, RandomX, RandomXVM};

pub const ALGOS: &[(&str, Algorithm)] = &[
  ("rx/0", Algorithm::RandomX),
];

#[derive(Clone)]
pub enum Algorithm {
  RandomX
}

#[derive(Clone)]
pub enum AlgorithmHasher {
  RandomX((RandomX, RandomXVM))
}