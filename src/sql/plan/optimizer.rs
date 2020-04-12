use super::super::types::{Environment, Expression};
use super::Node;
use crate::Error;

/// A plan optimizer
pub trait Optimizer {
    fn optimize(&mut self, node: Node) -> Result<Node, Error>;
}

/// A constant folding optimizer, which replaces constant expressions
/// with their evaluated value, to prevent it from being re-evaluated
/// over and over again during plan execution.
pub struct ConstantFolder;

impl Optimizer for ConstantFolder {
    fn optimize(&mut self, node: Node) -> Result<Node, Error> {
        let env = Environment::new();
        node.transform(&|n| Ok(n), &|n| {
            n.transform_expressions(&|e| Ok(e), &|e| {
                Ok(if e.is_constant() { Expression::Constant(e.evaluate(&env)?) } else { e })
            })
        })
    }
}