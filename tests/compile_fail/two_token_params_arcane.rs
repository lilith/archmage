//! #[arcane] allows multiple same-architecture tokens, but rejects combining
//! tokens from different architectures (here Scalar and x86_64).
#![forbid(unsafe_code)]
use archmage::prelude::*;

#[arcane]
fn probe(_s: ScalarToken, _v: X64V3Token) -> X64V3Token {
    X64V3Token::from_context()
}

fn main() {}
