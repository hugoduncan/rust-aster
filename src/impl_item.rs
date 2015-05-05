use syntax::abi::Abi;
use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span, respan};
use syntax::ptr::P;

use attr::AttrBuilder;
use expr::ExprBuilder;
use fn_decl::FnDeclBuilder;
use generics::GenericsBuilder;
use ident::ToIdent;
use invoke::{Invoke, Identity};
use mac::MacBuilder;
use method::MethodBuilder;
use path::PathBuilder;
use ty::TyBuilder;

/// A builder for items in an impl item.
pub struct ImplItemBuilder<F>
{
    callback: F,
    id: ast::NodeId,
    vis: ast::Visibility,
    attrs: Vec<ast::Attribute>,
    span: Span,
    // ident: Ident,
    // node: ImplItem_,
}


impl<F> ImplItemBuilder<F>
    where F: Invoke<P<ast::ImplItem>>,
{
    pub fn new_with_callback(callback: F, id: ast::NodeId) -> ImplItemBuilder<F> {
        ImplItemBuilder {
            callback:  callback,
            id: id,
            vis: ast::Visibility::Inherited,
            attrs: vec![],
            span: DUMMY_SP,
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn with_attr(mut self, attr: ast::Attribute) -> Self {
        self.attrs.push(attr);
        self
    }

    pub fn attr(self) -> AttrBuilder<Self> {
        AttrBuilder::new_with_callback(self)
    }

    pub fn pub_(mut self) -> Self {
        self.vis = ast::Visibility::Public;
        self
    }

    pub fn const_(self) -> ConstBuilder<Self> {
        ConstBuilder::new_with_callback(self)
    }

    pub fn method(self) -> MethodBuilder<Self> {
        MethodBuilder::new_with_callback(self)
    }

    pub fn ty(self) -> TyBuilder<Self> {
        TyBuilder::new_with_callback(self)
    }

    pub fn mac(self) -> MacBuilder<Self> {
        MacBuilder::new_with_callback(self)
    }


    fn build_impl_item<T>(self, impl_item: ast::ImplItem_) -> F::Result
        where T: ToIdent,
    {
        let item = ast::ImplItem {
            id: ast::DUMMY_NODE_ID,
            ident: self.id,
            vis: self.vis,
            attrs: self.attrs,
            node: impl_item,
            span: self.span,
        };
        self.callback.invoke(P(item))
    }
}

// pub enum ImplItem_ {

//     ConstImplItem(P<Ty>, P<Expr>),
//     MethodImplItem(MethodSig, P<Block>),

//     TypeImplItem(P<Ty>),
//     MacImplItem(Mac),
// }

impl<F> Invoke<ast::Attribute> for ImplItemBuilder<F>
    where F: Invoke<P<ast::ImplItem>>,
{
    type Result = Self;

    fn invoke(self, attr: ast::Attribute) -> Self {
        self.with_attr(attr)
    }
}

impl<F> Invoke<P<ast::Ty>> for ImplItemBuilder<F>
    where F: Invoke<P<ast::ImplItem>>,
{
    type Result = F::Result;

    fn invoke(self, ty: P<ast::Ty>) -> F::Result {
        self.build_impl_item(ast::ImplItem_::TypeImplItem(ty))
    }
}

impl<F> Invoke<ast::Mac> for ImplItemBuilder<F>
    where F: Invoke<P<ast::ImplItem>>,
{
    type Result = F::Result;

    fn invoke(self, mac: ast::Mac) -> F::Result {
        self.build_impl_item(ast::MacImplItem(mac))
    }
}

impl<F> Invoke<ast::ImplItem_> for ImplItemBuilder<F>
    where F: Invoke<P<ast::ImplItem>>,
{
    type Result = F::Result;

    fn invoke(self, item: ast::ImplItem_) -> F::Result {
        self.build_impl_item(item)
    }
}


//////////////////////////////////////////////////////////////////////////////

pub struct ConstBuilder<F> {
    builder: F,
}

impl<F> ConstBuilder<F>
    where F: Invoke<ast::ImplItem_>,
{
    pub fn new_with_callback(callback: F) -> TyBuilder<Self> {
        TyBuilder::new_with_callback(ConstBuilder {
            builder: callback,
        })
    }

    pub fn build_ty(self, ty: P<ast::Ty>) -> ConstExprBuilder<F> {
        ConstExprBuilder::new_with_callback(self.builder, ty)
    }
}

impl<F> Invoke<P<ast::Ty>> for ConstBuilder<F>
    where F: Invoke<ast::ImplItem_>,
{
    type Result = ConstExprBuilder<F>;

    fn invoke(self, ty: P<ast::Ty>) -> ConstExprBuilder<F> {
        self.build_ty(ty)
    }
}

//////////////////////////////////////////////////////////////////////////////

pub struct ConstExprBuilder<F> {
    builder: F,
    ty: P<ast::Ty>,
}

impl<F> ConstExprBuilder<F>
    where F: Invoke<ast::ImplItem_>,
{
    pub fn new_with_callback(callback: F, ty: P<ast::Ty>) -> Self {
        ConstExprBuilder {
            builder: callback,
            ty: ty,
        }
    }

    pub fn ty(self, ty: P<ast::Ty>) -> Self {
        self.ty = ty;
        self
    }

    pub fn expr(self) -> ExprBuilder<Self> {
        ExprBuilder::new_with_callback(self)
    }

    pub fn build_expr(self, expr: P<ast::Expr>) -> F::Result {
        self.builder.invoke(ast::ImplItem_::ConstImplItem(self.ty, expr))
    }
}


impl<F> Invoke<P<ast::Expr>> for ConstExprBuilder<F>
    where F: Invoke<ast::ImplItem_>,
{
    type Result = F::Result;

    fn invoke(self, expr: P<ast::Expr>) -> F::Result {
        self.build_expr(expr)
    }
}


// //////////////////////////////////////////////////////////////////////////////

// pub struct MethodBuilder<F> {
//     builder: F,
// }

// //  MethodImplItem(MethodSig, P<Block>),
// impl<F> MethodBuilder<F>
//     where F: Invoke<ast::MethodSig>,
// {
//     pub fn new_with_callback(callback: F) -> MethodSigBuilder<Self> {
//         MethodSigBuilder::new_with_callback(MethodBuilder {
//             callback: callback,
//         })
//     }
// }

// impl<F> Invoke<P<ast::MethodSig>> for MethodBuilder<F>
//     where F: Invoke<P<ast::MethodSig>>,
// {
//     type Result = MethodBlockBuilder<F>;

//     fn invoke(self, sig: P<ast::MethodSig>) -> MethodBlockBuilder<F> {
//         MethodBlockBuilder {
//             callback: self.callback,
//             sig: sig,
//         }
//     }
// }

// //////////////////////////////////////////////////////////////////////////////

// pub struct MethodBlockBuilder<F> {
//     builder: F,
//     sig: ast::MethodSig,
// }

// impl<F> MethodBlockBuilder<F>
//     where F: Invoke<ast::ImplItem_>,
// {
//     pub fn new_with_callback(
//         callback: F,
//         sig: ast::MethodSig,
//         ) -> ExprBuilder<MethodBlockBuilder<Self>>
//     {
//         ExprBuilder::new(MethodBlockBuilder {
//             builder: callback,
//             sig: sig,
//         })
//     }

//     pub fn build_expr(self, expr: P<ast::Expr>) -> ExprBuilder<Self> {
//         self.callback.invoke(ast::ImplItem_::MethodImplItem(self.sig, expr))
//     }
// }

// //////////////////////////////////////////////////////////////////////////////

// // pub struct MethodSig {
// //     pub unsafety: Unsafety,
// //     pub abi: Abi,
// //     pub decl: P<FnDecl>,
// //     pub generics: Generics,
// //     pub explicit_self: ExplicitSelf,
// // }

// pub struct MethodSigBuilder<F> {
//     builder: F,
//     unsafety: ast::Unsafety,
//     abi: Abi,
//     decl: P<ast::FnDecl>,
//     generics: ast::Generics,
//     explicit_self: ast::ExplicitSelf,
// }

// impl<F> MethodSigBuilder<F>
//     where F: Invoke<ast::MethodSig>,
// {
//     pub fn new_with_callback(callback: F) -> MethodSigBuilder<Self> {
//         let generics = GenericsBuilder::new().build();

//         MethodSigBuilder::new_with_callback(MethodBuilder {
//             callback: callback,
//             unsafety: ast::Unsafety::Normal,
//             abi: Abi::Rust,
//             generics: generics,
//             explicit_self: ast::ExplicitSelf::SelfStatic,
//         })
//     }

//     pub fn unsafe_(mut self) -> Self {
//         self.unsafety = ast::Unsafety::Normal;
//         self
//     }

//     pub fn abi(mut self, abi: Abi) -> Self {
//         self.abi = abi;
//         self
//     }

//     pub fn generics(self) -> GenericsBuilder<Self> {
//         GenericsBuilder::new_with_callback(self)
//     }

// }

// impl<F> Invoke<ast::Generics> for MethodSigBuilder<F>
//     where F: Invoke<P<ast::MethodSig>>,
// {
//     type Result = Self;

//     fn invoke(mut self, generics: ast::Generics) -> Self {
//         self.generics = generics;
//         self
//     }
// }

// //////////////////////////////////////////////////////////////////////////////


// // pub enum ExplicitSelf_ {
// //     SelfStatic,
// //     SelfValue(Ident),
// //     SelfRegion(Option<Lifetime>, Mutability, Ident),
// //     SelfExplicit(P<Ty>, Ident),
// // }

// // type ExplicitSelf = Spanned<ExplicitSelf_>;

// // pub enum Mutability {
// //     MutMutable,
// //     MutImmutable,
// // }
// pub struct ExplicitSelfBuilder<F> {
//     builder: F,
//     span: Span,
// }

// impl<F> ExplicitSelfBuilder<F>
//     where F: Invoke<ast::ExplicitSelf>,
// {
//     pub fn new_with_callback(callback: F) -> ExplicitSelfBuilder<F> {
//         ExplicitSelfBuilder {
//             builder: callback,
//             span: DUMMY_SP,
//         }
//     }

//     pub fn static_(self) -> F::Result {
//         self.builder.invoke(respan(self.span, ast::ExplicitSelf_::SelfStatic))
//     }

//     pub fn value<T>(self, id: T) -> F::Result
//         where T: ToIdent,
//     {
//         let id = id.to_ident();
//         self.builder.invoke(
//             respan(self.span, ast::ExplicitSelf_::SelfValue(id)))
//     }


//     pub fn region<T>(self, id: T) -> F::Result
//         where T: ToIdent,
//     {
//         let id = id.to_ident();
//         self.builder.invoke(
//             respan(self.span, ast::ExplicitSelf_::SelfRegion(id)))
//     }
// }
