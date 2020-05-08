use proc_macro2::{Literal, Span, TokenStream};
use quote::ToTokens;
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::str::FromStr;
use syn::{Error, Expr, Lit, Result};

pub struct DiscriminantSet {
    values: HashSet<Discriminant>,
    previous: Option<Discriminant>,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Discriminant {
    magnitude: u32,
}

impl DiscriminantSet {
    pub fn new() -> Self {
        DiscriminantSet {
            values: HashSet::new(),
            previous: None,
        }
    }

    pub fn insert(&mut self, expr: &Expr) -> Result<Discriminant> {
        let discriminant = expr_to_discriminant(expr)?;
        insert(self, discriminant)
    }

    pub fn insert_next(&mut self) -> Result<Discriminant> {
        let discriminant = match self.previous {
            None => Discriminant::zero(),
            Some(mut discriminant) => {
                if discriminant.magnitude == u32::MAX {
                    let msg = format!("discriminant overflow on value after {}", u32::MAX);
                    return Err(Error::new(Span::call_site(), msg));
                }
                discriminant.magnitude += 1;
                discriminant
            }
        };
        insert(self, discriminant)
    }
}

fn expr_to_discriminant(expr: &Expr) -> Result<Discriminant> {
    if let Expr::Lit(expr) = expr {
        if let Lit::Int(lit) = &expr.lit {
            return lit.base10_parse::<Discriminant>();
        }
    }
    Err(Error::new_spanned(
        expr,
        "enums with non-integer literal discriminants are not supported yet",
    ))
}

fn insert(set: &mut DiscriminantSet, discriminant: Discriminant) -> Result<Discriminant> {
    if set.values.insert(discriminant) {
        set.previous = Some(discriminant);
        Ok(discriminant)
    } else {
        let msg = format!("discriminant value `{}` already exists", discriminant);
        Err(Error::new(Span::call_site(), msg))
    }
}

impl Discriminant {
    fn zero() -> Self {
        Discriminant { magnitude: 0 }
    }
}

impl Display for Discriminant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.magnitude, f)
    }
}

impl ToTokens for Discriminant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Literal::u32_unsuffixed(self.magnitude).to_tokens(tokens);
    }
}

impl FromStr for Discriminant {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.parse::<u32>() {
            Ok(magnitude) => Ok(Discriminant { magnitude }),
            Err(_) => Err(Error::new(
                Span::call_site(),
                "discriminant value outside of supported range",
            )),
        }
    }
}
