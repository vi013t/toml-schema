use convert_case::Casing as _;

use crate::{property::Property, ChildValue, Value};

#[derive(Clone)]
pub(crate) struct InlineTable(pub Vec<Property>);

impl syn::parse::Parse for InlineTable {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut properties = Vec::new();
        let mut ate_last_comma = true;
        while input.peek(syn::Ident) {
            if !ate_last_comma {
                input.parse::<syn::Token![,]>()?;
            }
            properties.push(input.parse()?);
            if input.peek(syn::token::Comma) {
                input.parse::<syn::Token![,]>()?;
                ate_last_comma = true;
            }
        }

        Ok(InlineTable(properties))
    }
}

impl InlineTable {
    pub fn into_child_inline_table(
        self,
        name: syn::Ident,
        parent_name: syn::Ident,
    ) -> ChildInlineTable {
        ChildInlineTable {
            table: self,
            parent_name,
            name,
        }
    }
}

pub struct ChildInlineTable {
    table: InlineTable,
    parent_name: syn::Ident,
    name: syn::Ident,
}

impl quote::ToTokens for ChildInlineTable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(quote::quote! {});
    }
}

pub struct ChildInlineTablePrelude(pub ChildInlineTable);

impl std::ops::Deref for ChildInlineTablePrelude {
    type Target = ChildInlineTable;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl quote::ToTokens for ChildInlineTablePrelude {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let heading_type = quote::format_ident!(
            "{}{}",
            self.parent_name
                .to_string()
                .to_case(convert_case::Case::Pascal),
            self.name.to_string().to_case(convert_case::Case::Pascal)
        );

        let properties = self
            .table
            .0
            .iter()
            .map(|property| {
                let name = property.0.clone();
                match property.1 {
                    Value::Boolean(_) => quote::quote! { #name: bool },
                    Value::String(_) => quote::quote! { #name: String },
                    Value::Number(_) => quote::quote! { #name: f64 },
                    Value::InlineTable(_) => {
                        let type_name = quote::format_ident!(
                            "{}{}",
                            heading_type,
                            name.to_string().to_case(convert_case::Case::Pascal)
                        );
                        quote::quote! { #name: #type_name  }
                    }
                }
            })
            .collect::<Vec<_>>();

        tokens.extend(quote::quote! {
            pub struct #heading_type {
                #(
                    #properties
                ),*
            }
        });
    }
}

pub struct ChildInlineTableDefault(pub ChildInlineTable);

impl std::ops::Deref for ChildInlineTableDefault {
    type Target = ChildInlineTable;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl quote::ToTokens for ChildInlineTableDefault {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let heading_type = quote::format_ident!(
            "{}{}",
            self.parent_name
                .to_string()
                .to_case(convert_case::Case::Pascal),
            self.name.to_string().to_case(convert_case::Case::Pascal)
        );

        let properties = self
            .table
            .0
            .iter()
            .map(|property| {
                let name = property.0.clone();
                let value = property.1.clone();
                let child_value = ChildValue {
                    value,
                    property_name: name.clone(),
                    parent_name: heading_type.clone(),
                };
                quote::quote! { #name: #child_value }
            })
            .collect::<Vec<_>>();

        tokens.extend(quote::quote! {
            #heading_type {
                #(
                    #properties
                ),*
            }
        });
    }
}
