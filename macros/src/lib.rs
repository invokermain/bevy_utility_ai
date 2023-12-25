pub(crate) mod common;
mod input_system;
mod targeted_input_system;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn input_system(args: TokenStream, input: TokenStream) -> TokenStream {
    input_system::input_system(args, input)
        .unwrap_or_else(|err| err.into_compile_error().into())
}

#[proc_macro_attribute]
pub fn targeted_input_system(args: TokenStream, input: TokenStream) -> TokenStream {
    targeted_input_system::targeted_input_system(args, input)
        .unwrap_or_else(|err| err.into_compile_error().into())
}
