use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use venial::{parse_declaration, Declaration, Enum, Struct};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let add_hello_world = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello {}", stringify!(#name))
            }
        }
    };

    add_hello_world.into()
}

#[proc_macro_derive(HelloAlt)]
pub fn hello_alt(item: TokenStream) -> TokenStream {
    fn ident_name(item: TokenTree) -> String {
        match item {
            TokenTree::Ident(i) => i.to_string(),
            _ => panic!("No ident"),
        }
    }
    let name = ident_name(item.into_iter().nth(1).unwrap());

    format!(
        "impl {name} {{ fn hello_world(&self) \
    {{ println!(\"Hello world\") }} }} "
    )
    .parse()
    .unwrap()
}

#[proc_macro_derive(HelloVenial)]
pub fn hello_venial(item: TokenStream) -> TokenStream {
    let declaration = parse_declaration(item.into()).unwrap();

    let name = match declaration {
        Declaration::Struct(Struct { name, .. }) => name,
        Declaration::Enum(Enum { name, .. }) => name,
        _ => panic!("Only implemented for struct and enum"),
    };

    let add_hello_world = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello world")
            }
        }
    };

    add_hello_world.into()
}
