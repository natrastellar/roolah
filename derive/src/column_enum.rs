use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DeriveInput};

pub fn impl_column_enum(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let match_self = match input.data {
        Data::Enum(ref data) => {
            let arms = data.variants.iter().map(|v| {
                let variant_name = &v.ident;
                let snake_name = snake_case(&variant_name.to_string());
                match v.fields.len() {
                    0 => quote_spanned! {v.span()=>
                        #name::#variant_name => #snake_name
                    },
                    _ => quote_spanned! {v.span()=>
                        #name::#variant_name(_) => #snake_name
                    },
                }
            });

            quote! {
                match *self {
                    #(#arms,
                    )*
                }
            }
        }
        _ => unimplemented!(),
    };

    let expanded = quote! {
        impl roolah::ColumnEnum for #name {
            fn name(&self) -> &str {
                #match_self
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", (self as &dyn roolah::ColumnEnum).name())
            }
        }
    };

    expanded.into()
}

fn snake_case(s: &str) -> String {
    let mut result = String::new();
    s.chars().enumerate().for_each(|(i, c)| match (i, c) {
        (0, _) => result.push(c.to_ascii_lowercase()),
        (_, c) => {
            match c.is_ascii_uppercase() {
                true => {
                    result.push('_');
                    result.push(c.to_ascii_lowercase());
                }
                false => result.push(c),
            };
        }
    });
    result
}
