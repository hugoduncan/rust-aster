use syntax::ast;
use syntax::codemap::{self, DUMMY_SP, Span, respan};
use syntax::ext::base::ExtCtxt;
use syntax::ext::expand;
use syntax::ext::quote::rt::ToTokens;
use syntax::parse::{self, ParseSess};
use syntax::ptr::P;

use expr::ExprBuilder;
use invoke::{Invoke, Identity};

/// A Builder for macro invocations.
///
/// Note that there are no commas added between args, as otherwise
/// that macro invocations that could be expressed would be limited.
/// You will need to add all required symbols with `with_arg` or
/// `with_argss`.
pub struct MacBuilder<F=Identity> {
    callback: F,
    span: Span,
    tokens: Vec<ast::TokenTree>,
    path: Option<ast::Path>,
}

impl MacBuilder {
    pub fn new() -> Self {
        MacBuilder::new_with_callback(Identity)
    }
}

impl<F> MacBuilder<F>
    where F: Invoke<ast::Mac>
{
    pub fn new_with_callback(callback: F) -> Self {
        MacBuilder {
            callback: callback,
            span: DUMMY_SP,
            tokens: vec![],
            path: None,
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn path(mut self, path: ast::Path) -> Self {
        self.path = Some(path);
        self
    }

    pub fn build(self) -> F::Result {
        let mac = ast::Mac_::MacInvocTT(
            self.path.expect("No path set for macro"), self.tokens, 0);
        self.callback.invoke(respan(self.span, mac))
    }

    pub fn with_args<I, T>(self, iter: I) -> Self
        where I: IntoIterator<Item=T>, T: ToTokens
    {
        iter.into_iter().fold(self, |self_, expr| self_.with_arg(expr))
    }

    pub fn with_arg<T>(mut self, expr: T) -> Self
        where T: ToTokens
    {
        use syntax::ext::quote::rt::ToTokens;
        let parse_sess = parse::new_parse_sess();
        let cx = make_ext_ctxt(&parse_sess);
        let tokens = expr.to_tokens(&cx);
        assert!(tokens.len() == 1);
        self.tokens.push(tokens[0].clone());
        self
    }

    pub fn expr(self) -> ExprBuilder<Self> {
        ExprBuilder::new_with_callback(self)
    }

}

impl<F> Invoke<P<ast::Expr>> for MacBuilder<F>
    where F: Invoke<ast::Mac>,
{
    type Result = Self;

    fn invoke(self, expr: P<ast::Expr>) -> Self {
        self.with_arg(expr)
    }
}

fn make_ext_ctxt(sess: &ParseSess) -> ExtCtxt {
    let info = codemap::ExpnInfo {
        call_site: codemap::DUMMY_SP,
        callee: codemap::NameAndSpan {
            name: "test".to_string(),
            format: codemap::MacroAttribute,
            allow_internal_unstable: false,
            span: None
        }
    };

    let cfg = vec![];
    let ecfg = expand::ExpansionConfig {
        crate_name: String::new(),
        features: None,
        recursion_limit: 64,
        trace_mac: false,
    };

    let mut cx = ExtCtxt::new(&sess, cfg, ecfg);
    cx.bt_push(info);

    cx
}
