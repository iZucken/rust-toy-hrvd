extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{DataEnum, DataStruct, DataUnion, Ident};

fn derive_struct_diff(data_struct: &DataStruct, name: &Ident) -> TokenStream {
    let mut template = format!("{}:\n", name);
    let mut variables = vec![];

    for field in &data_struct.fields {
        let field_name = field.ident.as_ref().unwrap();
        template += &format!("\t{}: {}\n", field.ident.as_ref().unwrap().to_string(), "{}");
        variables.push(quote! {
            (&self.#field_name).diff_to(&other.#field_name)
        });
    }

    (quote! {
        impl Diffable for #name {
            fn diff_to(self: &#name, other: &#name) -> String {
                format!(#template, #(#variables),*)
            }
        }
    }).into()
}

fn derive_enum_diff(_data_struct: &DataEnum, name: &Ident) -> TokenStream {
    (quote! {
        impl Diffable for #name {
            fn diff_to(self: &#name, other: &#name) -> String {
                "uncomparable".to_string()
            }
        }
    }).into()
}

fn derive_union_diff(_data_struct: &DataUnion, name: &Ident) -> TokenStream {
    (quote! {
        impl Diffable for #name {
            fn diff_to(self: &#name, other: &#name) -> String {
                "uncomparable".to_string()
            }
        }
    }).into()
}

#[proc_macro_derive(Diffable)]
pub fn diffable(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let tokens = match &ast.data {
        syn::Data::Struct(x) => derive_struct_diff(x, &ast.ident),
        syn::Data::Enum(x) => derive_enum_diff(x, &ast.ident),
        syn::Data::Union(x) => derive_union_diff(x, &ast.ident),
    };
    eprintln!("{}", tokens);
    tokens
}
