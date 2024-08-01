use crate::common::traits::Initializer;
use crate::tree::core::individual::TreeGenotype;

pub struct Full {}

impl Initializer<TreeGenotype> for Full {
    fn initialize(&self) -> TreeGenotype {
        // Pseudocode
        // 0: initialize stack (Vec<bool> should suffice)
        // 1: initialize structure, e.g. Vec<id>
        // 2: push value onto stack
        // 3: while !stack.is_empty()
        // 4:   pop from stack and initialize (id, arity) pair
        // 5:   for arity:
        // 6:       push value onto stack
        // 7:   end
        // 8: end
        // 9: construct TreeGenotype via associated function (e.g. new)
        // 10: return created TreeGenotype
        //
        todo!()
    }
}
