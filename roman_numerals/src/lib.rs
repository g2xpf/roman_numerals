#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate rustc;
extern crate rustc_driver;
extern crate rustc_plugin;
extern crate syntax;
extern crate syntax_pos;

use rustc_plugin::Registry;
use syntax::ext::base::{DummyResult, ExtCtxt, MacEager, MacResult};
use syntax::parse::token;
use syntax::tokenstream::TokenStream;
use syntax::tokenstream::TokenTree;
use syntax_pos::Span;

fn expand_rn(cx: &mut ExtCtxt, sp: Span, args: TokenStream) -> Box<dyn MacResult + 'static> {
    static NUMERALS: &'static [(&'static str, usize)] = &[
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    if args.len() != 1 {
        cx.span_err(
            sp,
            &format!(
                "argument should be a single identifier, but got {} arguments",
                args.len()
            ),
        );
        return DummyResult::any(sp);
    }

    let mut total = 0;
    if let Some(args) = args.0 {
        let arg = &args[0];
        let text = match arg.0 {
            TokenTree::Token(token::Token {
                kind: token::Ident(s, _),
                ..
            }) => s.to_string(),
            _ => {
                cx.span_err(sp, "argument should be a single identifier");
                return DummyResult::any(sp);
            }
        };

        let mut text = &*text;
        // let mut total = 0;
        while !text.is_empty() {
            match NUMERALS.iter().find(|&&(rn, _)| text.starts_with(rn)) {
                Some(&(rn, val)) => {
                    total += val;
                    text = &text[rn.len()..];
                }
                None => {
                    cx.span_err(sp, "invalid Roman numeral");
                    return DummyResult::any(sp);
                }
            }
        }
    }

    MacEager::expr(cx.expr_usize(sp, total))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("rn", expand_rn);
}
