use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn root() -> TokenStream {
    quote! {
        ::agentdb_system::hidden
    }
}

#[proc_macro_derive(Agent)]
pub fn derive_agent(_item: TokenStream) -> TokenStream {
    let rt = root();

    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl #rt::Agent for
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
