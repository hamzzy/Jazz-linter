use crate::utils::Issue;
use swc_ecma_ast::Module;

mod no_unused_variables;
mod use_const;

pub use no_unused_variables::NoUnusedVariables;
pub use use_const::UseConstRule;
// pub use consistent_indentation::ConsistentIndentation;

pub trait Rule {
    fn check_module(&self, module: &Module) -> Vec<Issue>;
}