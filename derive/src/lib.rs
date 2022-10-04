use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod column_enum;

#[proc_macro_derive(ColumnEnum)]
pub fn column_enum(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    column_enum::impl_column_enum(input)
}
