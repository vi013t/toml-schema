use crate::item::Item;

pub struct TomlDocument {
    items: Vec<Item>,
}

impl syn::parse::Parse for TomlDocument {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            let item: Item = input.parse()?;
            match item {
                Item::Table(table) => items.push(Item::Table(table)),
                Item::Property(property) => items.push(Item::Property(property)),
            }
        }
        Ok(TomlDocument { items })
    }
}

impl quote::ToTokens for TomlDocument {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let parent_name = quote::format_ident!("Root");

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

            struct #parent_name {
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
        });
    }
}
