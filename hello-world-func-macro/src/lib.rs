use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn hello_world_func(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);

    quote!(
        impl #ident {
            fn hello() {
                println!("Hello world!")
            }
        }
    )
    .into()
}
