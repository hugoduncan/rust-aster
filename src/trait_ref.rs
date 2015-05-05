use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span, respan};
use syntax::ptr::P;

use ident::ToIdent;
use invoke::{Invoke, Identity};
use path::PathBuilder;
use ty::TyBuilder;

/// A TraitRef Builder
pub struct TraitRefBuilder<F=Identity> {
    callback: F,
    ref_id: ast::NodeId,
}

impl<F> TraitRefBuilder<F>
    where F: Invoke<ast::TraitRef>,
{
    pub fn new_with_callback(callback: F) -> PathBuilder<Self> {
        PathBuilder::new_with_callback(TraitRefBuilder {
            callback:  callback,
            ref_id: ast::DUMMY_NODE_ID,
        })
    }

    pub fn path(self) -> PathBuilder<Self> {
        PathBuilder::new_with_callback(self)
    }

    fn build_path(self, path: ast::Path) -> F::Result {
        self.callback.invoke( ast::TraitRef {
            path: path,
            ref_id: self.ref_id,
        })
    }
}

impl<F> Invoke<ast::Path> for TraitRefBuilder<F>
    where F: Invoke<ast::TraitRef>,
{
    type Result = F::Result;

    fn invoke(mut self, path: ast::Path) -> F::Result {
        self.build_path(path)
    }
}
