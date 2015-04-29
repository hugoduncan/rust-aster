#![cfg_attr(not(feature = "syntex"), feature(rustc_private))]

#[cfg(feature = "syntex")]
extern crate syntex_syntax as syntax;

#[cfg(not(feature = "syntex"))]
extern crate syntax;

extern crate aster;

use syntax::ast;
use syntax::owned_slice::OwnedSlice;

use aster::AstBuilder;

#[test]
fn test_empty() {
    let builder = AstBuilder::new();
    let generics = builder.generics().build();

    assert_eq!(
        generics,
        ast::Generics {
            lifetimes: vec![],
            ty_params: OwnedSlice::empty(),
            where_clause: ast::WhereClause {
                id: ast::DUMMY_NODE_ID,
                predicates: vec![],
            },
        }
    );
}

#[test]
fn test_with_ty_params_and_lifetimes() {
    let builder = AstBuilder::new();
    let generics = builder.generics()
        .lifetime("'a").build()
        .lifetime("'b").bound("'a").build()
        .ty_param("T").lifetime_bound("'a").build()
        .build();

    assert_eq!(
        generics,
        ast::Generics {
            lifetimes: vec![
                builder.lifetime_def("'a").build(),
                builder.lifetime_def("'b").bound("'a").build(),
            ],
            ty_params: OwnedSlice::from_vec(vec![
                builder.ty_param("T").lifetime_bound("'a").build(),
            ]),
            where_clause: ast::WhereClause {
                id: ast::DUMMY_NODE_ID,
                predicates: vec![],
            },
        }
    );
}
