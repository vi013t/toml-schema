use convert_case::Casing as _;
use quote::format_ident;

use crate::{
    inline_table::ChildInlineTablePrelude,
    property::{ChildProperty, ChildPropertyDefault},
    TableContents, Value,
};

#[derive(Clone)]
pub(crate) struct Table {
    heading: syn::Ident,
    contents: TableContents,
}

impl syn::parse::Parse for Table {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let heading;
        syn::bracketed!(heading in input);
        let heading: syn::Ident = heading.parse()?;
        let contents: TableContents = input.parse()?;

        Ok(Table { heading, contents })
    }
}

impl Table {
    pub fn into_child_table(self, parent_name: syn::Ident) -> ChildTable {
        ChildTable {
            table: self,
            parent_name,
        }
    }
}

#[derive(Clone)]
pub struct ChildTable {
    table: Table,
    parent_name: syn::Ident,
}

impl quote::ToTokens for ChildTable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let heading_type = quote::format_ident!(
            "{}{}",
            self.parent_name
                .to_string()
                .to_case(convert_case::Case::Pascal),
            self.table
                .heading
                .to_string()
                .to_case(convert_case::Case::Pascal)
        );
        let heading = &self.table.heading;
        tokens.extend(quote::quote! {
            #heading: #heading_type
        });
    }
}

/// The implementation for `quote::ToTokens` for this struct creates the tokens for defining the struct for the
/// type of this table.
#[derive(Clone)]
pub struct ChildTablePrelude(pub ChildTable);

impl std::ops::Deref for ChildTablePrelude {
    type Target = ChildTable;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl quote::ToTokens for ChildTablePrelude {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let heading_type = quote::format_ident!(
            "{}{}",
            self.parent_name
                .to_string()
                .to_case(convert_case::Case::Pascal),
            self.table
                .heading
                .to_string()
                .to_case(convert_case::Case::Pascal)
        );

        let properties = self
            .table
            .contents
            .0
            .iter()
            .map(|property| {
                let name = property.0.clone();
                match property.1.clone() {
                    Value::Boolean(_) => quote::quote! { #name: bool },
                    Value::String(_) => quote::quote! { #name: String },
                    Value::Number(_) => quote::quote! { #name: f64 },
                    Value::InlineTable(inline_table) => {
                        ChildInlineTablePrelude(
                            inline_table
                                .into_child_inline_table(name.clone(), heading_type.clone()),
                        )
                        .to_tokens(tokens);
                        let type_name = format_ident!(
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
            struct #heading_type {
                #(
                    #properties
                ),*
            }
        });
    }
}

#[derive(Clone)]
pub struct ChildTableDefault(pub ChildTable);

impl std::ops::Deref for ChildTableDefault {
    type Target = ChildTable;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl quote::ToTokens for ChildTableDefault {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let heading_type = quote::format_ident!(
            "{}{}",
            self.parent_name
                .to_string()
                .to_case(convert_case::Case::Pascal),
            self.table
                .heading
                .to_string()
                .to_case(convert_case::Case::Pascal)
        );

        let properties = self
            .table
            .contents
            .0
            .iter()
            .map(|property| {
                let property = ChildPropertyDefault(ChildProperty {
                    parent_name: heading_type.clone(),
                    property: property.clone(),
                });
                quote::quote! { #property }
            })
            .collect::<Vec<_>>();

        let heading = self.table.heading.clone();

        tokens.extend(quote::quote! {
            #heading: #heading_type {
                #(
                    #properties
                ),*
            }
        });
    }
}
