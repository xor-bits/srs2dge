use parse::DeriveParsed;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

//

mod codegen;
mod parse;

//

#[proc_macro_derive(Widget, attributes(gui))]
pub fn derive_widget(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let derive = DeriveParsed::new(&input).unwrap_or_else(|err| panic!("{err}"));

    (quote! {
        #derive
    })
    .into()
}
