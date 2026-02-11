use syn::{visit::Visit, ItemStruct}; // Removed 'Type'
use anyhow::{Result, bail};

#[derive(Debug, Default)]
pub struct StateManifest {
    pub fields: Vec<String>,
}

struct StateVisitor {
    found_state: bool,
    manifest: StateManifest,
}

impl<'ast> Visit<'ast> for StateVisitor {
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        if node.ident == "State" {
            self.found_state = true;
            for field in &node.fields {
                if let Some(ident) = &field.ident {
                    self.manifest.fields.push(ident.to_string());
                }
            }
        }
        syn::visit::visit_item_struct(self, node);
    }
}

pub fn extract_state(ast: &syn::File) -> Result<StateManifest> {
    let mut visitor = StateVisitor {
        found_state: false,
        manifest: StateManifest::default(),
    };

    visitor.visit_file(ast);

    if !visitor.found_state {
        bail!("Sharding Error: No 'struct State' found in source code.");
    }

    Ok(visitor.manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_file;

    #[test]
    fn test_extract_state_fields() {
        let code = r#"
            struct State {
                balance: u64,
                username: String,
            }
            fn main() {}
        "#;
        let ast = parse_file(code).unwrap();
        let manifest = extract_state(&ast).unwrap();
        assert_eq!(manifest.fields.len(), 2);
    }
    
    #[test]
    fn test_fail_if_no_state() {
        let code = "fn main() { let x = 5; }";
        let ast = parse_file(code).unwrap();
        let result = extract_state(&ast);
        assert!(result.is_err());
    }
}
