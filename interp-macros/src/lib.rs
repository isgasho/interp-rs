extern crate proc_macro;

mod dissect;
mod error;
mod expand;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

use crate::error::{Error, Result};

#[proc_macro_hack]
pub fn interp(input: TokenStream) -> TokenStream {
    fn inner(input: TokenStream) -> Result<TokenStream> {
        let string: syn::LitStr = syn::parse(input).map_err(Error::Syn)?;
        let span = string.span();
        let string = string.value();
        let context = dissect::dissect(&string, span)?;
        let expanded = expand::expand(&context)?;
        println!("expanded: {:#?}", expanded);
        println!("expanded stream: {:#?}", proc_macro2::TokenStream::from(expanded.clone()));
        Ok(expanded.into())
    }
    inner(input).expect("Error expanding macro")
}
