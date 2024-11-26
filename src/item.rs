use crate::{
    property::{ChildProperty, ChildPropertyDefault, ChildPropertyPrelude, Property},
    table::{ChildTable, ChildTableDefault, ChildTablePrelude, Table},
};

#[derive(Clone)]
pub(crate) enum Item {
    Table(Table),
    Property(Property),
}

impl syn::parse::Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Bracket) {
            let table: Table = input.parse()?;
            return Ok(Item::Table(table));
        }

        if input.peek(syn::Ident) {
            let property: Property = input.parse()?;
            input.parse::<syn::Token![;]>()?;
            return Ok(Item::Property(property));
        }

        Err(input.error("Invalid TOML syntax"))
    }
}

impl Item {
    pub fn into_child_item(self, parent_name: syn::Ident) -> ChildItem {
        match self {
            Item::Table(table) => ChildItem::Table(table.into_child_table(parent_name)),
            Item::Property(property) => {
                ChildItem::Property(property.into_child_property(parent_name))
            }
        }
    }
}

#[derive(Clone)]
pub enum ChildItem {
    Table(ChildTable),
    Property(ChildProperty),
}

impl quote::ToTokens for ChildItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            ChildItem::Table(table) => table.to_tokens(tokens),
            ChildItem::Property(property) => property.to_tokens(tokens),
        }
    }
}

impl ChildItem {
    pub fn into_item_prelude(self) -> ChildItemPrelude {
        match self {
            ChildItem::Table(table) => ChildItemPrelude::Table(ChildTablePrelude(table)),
            ChildItem::Property(property) => {
                ChildItemPrelude::Property(ChildPropertyPrelude(property))
            }
        }
    }

    pub fn into_item_default(self) -> ChildItemDefault {
        match self {
            ChildItem::Table(table) => ChildItemDefault::Table(ChildTableDefault(table)),
            ChildItem::Property(property) => {
                ChildItemDefault::Property(ChildPropertyDefault(property))
            }
        }
    }
}

#[derive(Clone)]
pub enum ChildItemPrelude {
    Table(ChildTablePrelude),
    Property(ChildPropertyPrelude),
}

impl quote::ToTokens for ChildItemPrelude {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            ChildItemPrelude::Table(table) => table.to_tokens(tokens),
            ChildItemPrelude::Property(property) => property.to_tokens(tokens),
        }
    }
}

#[derive(Clone)]
pub enum ChildItemDefault {
    Table(ChildTableDefault),
    Property(ChildPropertyDefault),
}

impl quote::ToTokens for ChildItemDefault {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            ChildItemDefault::Table(table) => table.to_tokens(tokens),
            ChildItemDefault::Property(property) => property.to_tokens(tokens),
        }
    }
}
