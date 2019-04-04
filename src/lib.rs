extern crate quote;
extern crate proc_macro;

#[proc_macro]
pub fn quine(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let lib = String::from_utf8(std::fs::read("src/lib.rs").unwrap()).unwrap();
    let main = String::from_utf8(std::fs::read("src/main.rs").unwrap()).unwrap();
    quote::quote!(concat!(#lib, "\n", #main)).into()
}

