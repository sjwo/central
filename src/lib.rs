use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(IterStruct)]
pub fn iter_struct_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let field_names = if let Data::Struct(data_struct) = &input.data {
        match &data_struct.fields {
            Fields::Named(fields_named) => fields_named
                .named
                .iter()
                .map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    quote! { stringify!(#ident) }
                })
                .collect::<Vec<_>>(),
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "IterStruct can only be derived for structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        }
    } else {
        return syn::Error::new_spanned(&input, "IterStruct can only be derived for structs")
            .to_compile_error()
            .into();
    };

    let expanded = quote! {
        impl #struct_name {
            pub fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[
                    #(#field_names),*
                ];
                NAMES
            }
        }
    };

    TokenStream::from(expanded)
}
