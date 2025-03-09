use swc_atoms::Atom;
use swc_core::ecma::{
    ast::*,
    visit::{visit_mut_pass, VisitMut, VisitMutWith},
};

use swc_core::{
    ecma::ast::Program,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

struct RemoveNodeProtocolPlugin;

impl RemoveNodeProtocolPlugin {
    fn strip_node_prefix(atom: &mut Atom) {
        if let Some(new_value) = atom.as_ref().strip_prefix("node:") {
            *atom = Atom::from(new_value);
        }
    }
    pub fn new() -> Self {
        Self
    }
}

impl VisitMut for RemoveNodeProtocolPlugin {
    fn visit_mut_import_decl(&mut self, import: &mut ImportDecl) {
        Self::strip_node_prefix(&mut import.src.value);
        import.src.raw = Some(Atom::from(format!("'{}'", import.src.value)));
    }

    fn visit_mut_export_all(&mut self, export: &mut ExportAll) {
        Self::strip_node_prefix(&mut export.src.value);
        export.src.raw = Some(Atom::from(format!("'{}'", export.src.value)));
    }

    fn visit_mut_named_export(&mut self, export: &mut NamedExport) {
        if let Some(src) = &mut export.src {
            Self::strip_node_prefix(&mut src.value);
            src.raw = Some(Atom::from(format!("'{}'", src.value)));
        }
    }

    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        //import("node:xxx")
        if let Callee::Import(_) = &call.callee {
            if let Some(arg) = call.args.first_mut() {
                if let Expr::Lit(Lit::Str(str_lit)) = &mut *arg.expr {
                    Self::strip_node_prefix(&mut str_lit.value);
                    str_lit.raw = Some(Atom::from(format!("'{}'", str_lit.value)));
                }
            }
        }
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(Ident { sym, .. }) = &**expr {
                if sym == "require" {
                    if let Some(arg) = call.args.first_mut() {
                        if let Expr::Lit(Lit::Str(str_lit)) = &mut *arg.expr {
                            Self::strip_node_prefix(&mut str_lit.value);
                            str_lit.raw = Some(Atom::from(format!("'{}'", str_lit.value)));
                        }
                    }
                }
            } else if let Expr::Member(MemberExpr { obj, prop, .. }) = &**expr {
                if let Expr::Ident(Ident { sym: obj_sym, .. }) = &**obj {
                    if obj_sym == "require" {
                        if let MemberProp::Ident(IdentName { sym: prop_sym, .. }) = prop {
                            if prop_sym == "resolve" {
                                if let Some(arg) = call.args.first_mut() {
                                    if let Expr::Lit(Lit::Str(str_lit)) = &mut *arg.expr {
                                        Self::strip_node_prefix(&mut str_lit.value);
                                        str_lit.raw =
                                            Some(Atom::from(format!("'{}'", str_lit.value)));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn visit_mut_program(&mut self, program: &mut Program) {
        program.visit_mut_children_with(self);
    }
}

pub fn remove_node_protocol() -> impl Pass {
    visit_mut_pass(RemoveNodeProtocolPlugin::new())
}

#[plugin_transform]
fn swc_plugin(program: Program, _data: TransformPluginProgramMetadata) -> Program {
    program.apply(remove_node_protocol())
}

#[cfg(test)]
mod tests {
    use super::*;
    use swc_core::ecma::transforms::testing::test;
    test!(
        Default::default(),
        |_| visit_mut_pass(RemoveNodeProtocolPlugin),
        remove_node_protocol,
        r#"
    import fs from 'node:fs';
    import path from 'node:path';
    
    const dynamic = import('node:url');
    const fs = require('node:fs');
    const path = require.resolve('node:path');
    
    export * as fs from 'node:fs';
    export * from 'node:crypto';
    export { readFile } from 'node:fs';
    
    module.exports = { fs: require('node:fs') };
    module.exports.fs = require('node:fs');
    
    console.log('node:fs');
    fn('node:fs')
    "#
    );
}
