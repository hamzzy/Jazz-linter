use super::Rule;
use crate::utils::Issue;
use swc_ecma_ast::{Module, Ident, Expr};
use swc_ecma_visit::{Visit, VisitWith};
use std::collections::HashMap;
use std::rc::Rc;

pub struct NoUnusedVariables {
    declared: HashMap<String, bool>,
    used: HashMap<String, bool>,
}

impl Default for NoUnusedVariables {
    fn default() -> Self {
        NoUnusedVariables {
            declared: HashMap::new(),
            used: HashMap::new(),
        }
    }
}

impl Visit for NoUnusedVariables {
    fn visit_var_decl(&mut self, n: &swc_ecma_ast::VarDecl) {
        for decl in &n.decls {
            if let Some(ident) = decl.name.as_ident() {
                self.declared.insert(ident.sym.to_string(), true);
            }
        }
    }

    fn visit_ident(&mut self, n: &Ident) {
        self.used.insert(n.sym.to_string(), true);
    }

    fn visit_expr(&mut self, n: &Expr) {
        if let Expr::Ident(ident) = n {
            self.used.insert(ident.sym.to_string(), true);
        }
        n.visit_children_with(self);
    }
}

impl Rule for NoUnusedVariables {
    fn check_module(&self, module: &Module) -> Vec<Issue> {
        let mut checker = NoUnusedVariables::default();
        module.visit_with(&mut checker);
        
        checker.declared.iter()
            .filter(|(name, _)| !checker.used.contains_key(*name))
            .map(|(name, _)| Issue {
                rule: "no-unused-variables".to_string(),
                message: format!("Unused variable: {}", name),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
    use swc_common::{SourceMap, FileName};

    fn parse_module(code: &str) -> Module {
        let cm = SourceMap::default();
        let fm = cm.new_source_file(Rc::new(FileName::Anon), code.into());
        let lexer = Lexer::new(
            Syntax::Es(Default::default()),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );
        let mut parser = Parser::new_from(lexer);
        parser.parse_module().expect("failed to parse module")
    }

    #[test]
    fn test_no_unused_variables() {
        let code = r#"
            let x = 5;
            let y = 10;
            console.log(x);
        "#;
        let module = parse_module(code);
        let rule = NoUnusedVariables::default();
        let issues = rule.check_module(&module);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].message, "Unused variable: y");
    }
}