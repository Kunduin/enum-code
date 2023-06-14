use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn parse_code_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let getters: Vec<_> = match input.data {
        Data::Enum(e) => e
            .variants
            .iter()
            .map(|variant| {
                let attrs = variant.attrs.clone();
                let variant_ident = variant.ident.clone();
                let variant_fields = variant.fields.clone();

                let code_attr = attrs.iter().find_map(|attr| {
                    if attr.path().is_ident("code") {
                        let code_in_attr = attr
                            .parse_args::<syn::LitInt>()
                            .expect("#[code()] value must be integer")
                            .base10_parse::<u32>()
                            .expect("#[code()] value is not a integer");
                        Some(code_in_attr)
                    } else {
                        None
                    }
                });
                let code_value = code_attr.unwrap_or(0);

                match variant_fields {
                    Fields::Named(..) => quote! {
                        #name::#variant_ident { .. } => #code_value
                    },
                    Fields::Unnamed(..) => quote! {
                        #name::#variant_ident ( .. ) => #code_value
                    },
                    Fields::Unit => quote! {
                        #name::#variant_ident => #code_value
                    },
                }
            })
            .collect(),
        _ => panic!("Code attribute is only applicable to enums!"),
    };

    let output = quote! {
        impl #name {
            pub const fn get_code(&self) -> u32 {
                match self {
                    #(#getters),*
                }
            }
        }
    };
    proc_macro::TokenStream::from(output)
}
