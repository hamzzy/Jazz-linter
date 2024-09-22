use super::Rule;
use crate::utils::Issue;
use swc_ecma_ast::{Module, VarDecl, VarDeclKind, Pat};
use swc_ecma_visit::{Visit, VisitWith};
use std::rc::Rc;

pub struct UseConstRule {
    issues: Vec<Issue>,
}

impl Default for UseConstRule {
    fn default() -> Self {
        UseConstRule { issues: Vec::new() }
    }
}

impl Visit for UseConstRule {
    fn visit_var_decl(&mut self, n: &VarDecl) {
        if n.kind == VarDeclKind::Let {
            for decl in &n.decls {
                if let Pat::Ident(ident) = &decl.name {
                    if decl.init.is_some() {
                        self.issues.push(Issue {
                            rule: "use-const".to_string(),
                            message: format!("Use 'const' instead of 'let' for '{}' as it's not reassigned.", ident.id.sym),
                        });
                    }
                }
            }
        }
    }
}

impl Rule for UseConstRule {
    fn check_module(&self, module: &Module) -> Vec<Issue> {
        let mut checker = UseConstRule::default();
        module.visit_with(&mut checker);
        checker.issues
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
    fn test_use_const() {
        let code = r#"
            let x = 5;
            let y;
            const z = 10;
        "#;
        let module = parse_module(code);
        let rule = UseConstRule::default();
        let issues = rule.check_module(&module);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].message, "Use 'const' instead of 'let' for 'x' as it's not reassigned.");
    }
}