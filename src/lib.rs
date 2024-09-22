use wasm_bindgen::prelude::*;
use swc_common::SourceMap;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use std::rc::Rc;

mod rules;
mod utils;

use rules::Rule;
use utils::Issue;

#[wasm_bindgen]
pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

#[wasm_bindgen]
impl Linter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Linter {
            rules: vec![
                Box::new(rules::NoUnusedVariables::default()),
                Box::new(rules::UseConstRule::default()),
                // Box::new(rules::ConsistentIndentation::default()),
            ],
        }
    }

    pub fn lint(&self, code: &str) -> String {
        let cm = SourceMap::default();
        let fm = cm.new_source_file(Rc::new(swc_common::FileName::Anon), code.into());

        let lexer = Lexer::new(
            Syntax::Es(Default::default()),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);
        let module = parser.parse_module().expect("Failed to parse module");

        let mut issues = Vec::new();
        for rule in &self.rules {
            issues.extend(rule.check_module(&module));
        }

        serde_json::to_string(&issues).unwrap_or_else(|_| "[]".to_string())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_linter() {
//         let linter = Linter::new();
//         let code = r#"
//         let x = 5;
//         let y = 10;
//          console.log(x);
//         "#;
//         let result = linter.lint(code);
//         let issues: Vec<Issue> = serde_json::from_str(&result).unwrap();
//         assert_eq!(issues.len(), 3); // One for unused 'y', one for 'let' instead of 'const', and one for inconsistent indentation
//     }
// }