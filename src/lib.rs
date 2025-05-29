use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(StructKeys)]
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
                    "StructKeys can only be derived for structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        }
    } else {
        return syn::Error::new_spanned(&input, "StructKeys can only be derived for structs")
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

#[proc_macro_derive(StructKeysValues)]
pub fn struct_keys_values_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let (field_names, _field_idents) = if let Data::Struct(data_struct) = &input.data {
        match &data_struct.fields {
            Fields::Named(fields_named) => {
                let field_names = fields_named
                    .named
                    .iter()
                    .map(|f| {
                        let ident = f.ident.as_ref().unwrap();
                        quote! { stringify!(#ident) }
                    })
                    .collect::<Vec<_>>();
                let field_idents = fields_named
                    .named
                    .iter()
                    .map(|f| {
                        let ident = f.ident.as_ref().unwrap();
                        quote! { &self.#ident }
                    })
                    .collect::<Vec<_>>();
                (field_names, field_idents)
            }
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "StructKeysValues can only be derived for structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        }
    } else {
        return syn::Error::new_spanned(&input, "StructKeysValues can only be derived for structs")
            .to_compile_error()
            .into();
    };

    let field_string_lines = if let Data::Struct(data_struct) = &input.data {
        match &data_struct.fields {
            Fields::Named(fields_named) => {
                let lines = fields_named.named.iter().map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    let name_str = ident.to_string();
                    quote! {
                        s.push_str(&format!("{}: {:?}\n", #name_str, &self.#ident));
                    }
                });
                quote! {
                    let mut s = String::new();
                    #(#lines)*
                    s
                }
            }
            _ => quote! { String::new() },
        }
    } else {
        quote! { String::new() }
    };

    let expanded = quote! {
        impl #struct_name {
            pub fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[
                    #(#field_names),*
                ];
                NAMES
            }
            pub fn fields_string(&self) -> String {
                #field_string_lines
            }
        }
    };

    TokenStream::from(expanded)
}
//pub fn struct_keys_values_derive(input: TokenStream) -> TokenStream {
//    let input = parse_macro_input!(input as DeriveInput);
//
//    let struct_name = &input.ident;
//
//    let (field_names, field_idents) = if let Data::Struct(data_struct) = &input.data {
//        match &data_struct.fields {
//            Fields::Named(fields_named) => {
//                let field_names = fields_named.named.iter().map(|f| {
//                    let ident = f.ident.as_ref().unwrap();
//                    quote! { stringify!(#ident) }
//                }).collect::<Vec<_>>();
//                let field_idents = fields_named.named.iter().map(|f| {
//                    let ident = f.ident.as_ref().unwrap();
//                    quote! { &self.#ident }
//                }).collect::<Vec<_>>();
//                (field_names, field_idents)
//            }
//            _ => {
//                return syn::Error::new_spanned(
//                    &input,
//                    "StructKeysValues can only be derived for structs with named fields",
//                ).to_compile_error().into();
//            }
//        }
//    } else {
//        return syn::Error::new_spanned(
//            &input,
//            "StructKeysValues can only be derived for structs",
//        ).to_compile_error().into();
//    };
//
//    let field_prints = if let Data::Struct(data_struct) = &input.data {
//        match &data_struct.fields {
//            Fields::Named(fields_named) => {
//                let prints = fields_named.named.iter().map(|f| {
//                    let ident = f.ident.as_ref().unwrap();
//                    let name_str = ident.to_string();
//                    quote! {
//                        println!("{}: {:?}", #name_str, &self.#ident);
//                    }
//                });
//                quote! {
//                    #(#prints)*
//                }
//            }
//            _ => quote! {},
//        }
//    } else {
//        quote! {}
//    };
//
//    let expanded = quote! {
//        impl #struct_name {
//            pub fn field_names() -> &'static [&'static str] {
//                static NAMES: &'static [&'static str] = &[
//                    #(#field_names),*
//                ];
//                NAMES
//            }
//            pub fn print_fields(&self) {
//                #field_prints
//            }
//        }
//    };
//
//    TokenStream::from(expanded)
//}
