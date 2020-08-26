use proc_macro::TokenStream;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    println!("Derive input: {}", input);
    TokenStream::new()
}
