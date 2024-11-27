use quote::ToTokens;
use syn::bracketed;

use crate::item::Item;

pub struct TomlDocument {
    items: Vec<Item>,
    metadata: DocumentMetadata,
}

impl syn::parse::Parse for TomlDocument {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Metadata
        input.parse::<syn::Token![#]>()?;
        let meta;
        bracketed!(meta in input);
        let metadata = meta.parse()?;

        let mut items = Vec::new();
        while !input.is_empty() {
            let item: Item = input.parse()?;
            match item {
                Item::Table(table) => items.push(Item::Table(table)),
                Item::Property(property) => items.push(Item::Property(property)),
            }
        }
        Ok(TomlDocument { items, metadata })
    }
}

impl quote::ToTokens for TomlDocument {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let parent_name = self.metadata.name.clone();

        let items = self
            .items
            .clone()
            .into_iter()
            .map(|item| item.into_child_item(parent_name.clone()))
            .collect::<Vec<_>>();

        let prelude = items
            .clone()
            .into_iter()
            .map(|item| item.into_item_prelude())
            .collect::<Vec<_>>();

        let defaults = items
            .clone()
            .into_iter()
            .map(|item| item.into_item_default())
            .collect::<Vec<_>>();

        tokens.extend(quote::quote! {
            #(
                #prelude
            )*

            pub struct #parent_name {
                #(
                    #items
                ),*
            }

            impl Default for #parent_name {
                fn default() -> #parent_name {
                    #parent_name {
                        #(
                            #defaults
                        ),*
                    }
                }
            }

            impl #parent_name {
                pub fn try_set(&mut self, name: &str, value: &str) {

                }
            }
        });
    }
}

struct DocumentMetadata {
    name: syn::Ident,
}

impl syn::parse::Parse for DocumentMetadata {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key_token = input.parse::<syn::Ident>()?;
        if key_token.to_string() != "name" {
            return Err(input.error("Expected name"));
        }
        input.parse::<syn::Token![=]>()?;
        let name = input.parse()?;
        Ok(DocumentMetadata { name })
    }
}
