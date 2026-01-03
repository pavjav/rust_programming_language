use proc_macro::TokenStream;
use quote::quote;
use syn;


/// This is an example of a proc macro for deriving a trait from metadata about the struct
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen_ = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!(
                    "Hello, my name is {}",
                    stringify!(#name)
                )
            }
        }
    };
    gen_.into()
}

/// This is an example of an attribute-like macro
#[proc_macro_attribute]
pub fn route(
    attr: TokenStream, // This is for the parameters GET and "/"
    item: TokenStream, // This is for the item under 
) -> TokenStream
{
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}

/// This is an example of a function like macro

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    println!("Your SQL query: {input}");
    input
}