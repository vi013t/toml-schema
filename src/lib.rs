use document::TomlDocument;
use inline_table::{ChildInlineTableDefault, InlineTable};
use proc_macro::TokenStream;
use property::Property;
use syn::{parse::ParseStream, Lit};

mod document;
mod inline_table;
mod item;
mod property;
mod table;

#[proc_macro]
pub fn toml(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as TomlDocument);

    quote::quote! {
        #input
    }
    .into()
}

#[derive(Clone)]
struct TableContents(Vec<Property>);

impl syn::parse::Parse for TableContents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut properties = Vec::new();
        while input.peek(syn::Ident) {
            properties.push(input.parse()?);
            input.parse::<syn::Token![;]>()?;
        }

        Ok(TableContents(properties))
    }
}

impl TryInto<Value> for syn::Lit {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Value, Self::Error> {
        Ok(match self {
            Lit::Bool(value) => value.value.into(),
            Lit::Str(value) => value.value().into(),
            Lit::Int(value) => value.base10_digits().parse::<f64>()?.into(),
            Lit::Float(value) => value.base10_digits().parse::<f64>()?.into(),
            _ => anyhow::bail!("Invalid literal used as TOML value"),
        })
    }
}

#[derive(try_as::macros::From, Clone)]
enum Value {
    Boolean(bool),
    Number(f64),
    String(String),
    InlineTable(InlineTable),
}

struct ChildValue {
    value: Value,
    parent_name: syn::Ident,
    property_name: syn::Ident,
}

impl quote::ToTokens for ChildValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.value {
            Value::Boolean(value) => tokens.extend(quote::quote! { #value }),
            Value::String(value) => tokens.extend(quote::quote! { #value.into() }),
            Value::Number(value) => tokens.extend(quote::quote! { #value }),
            Value::InlineTable(value) => {
                let default =
                    ChildInlineTableDefault(value.clone().into_child_inline_table(
                        self.property_name.clone(),
                        self.parent_name.clone(),
                    ));
                tokens.extend(quote::quote! { #default })
            }
        }
    }
}
