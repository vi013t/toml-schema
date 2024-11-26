use convert_case::Casing;

use crate::{inline_table::ChildInlineTablePrelude, ChildValue, InlineTable, Value};

#[derive(Clone)]
pub(crate) struct Property(pub syn::Ident, pub Value);

impl syn::parse::Parse for Property {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        input.parse::<syn::Token![=]>()?;

        // Inline table
        if input.peek(syn::token::Brace) {
            let contents;
            syn::braced!(contents in input);
            let contents: InlineTable = contents.parse()?;
            return Ok(Property(name, Value::InlineTable(contents)));
        }

        // Literal value
        let field_value = match input.parse::<syn::Expr>()? {
            syn::Expr::Lit(literal) => {
                let value: Value = literal
                    .lit
                    .try_into()
                    .map_err(|_error| input.error("Invalid value used as a TOML value"))?;
                value
            }
            _ => return Err(input.error("Invalid value for a TOML value")),
        };

        Ok(Property(name, field_value))
    }
}

impl Property {
    pub fn into_child_property(self, parent_name: syn::Ident) -> ChildProperty {
        ChildProperty {
            property: self,
            parent_name,
        }
    }
}

#[derive(Clone)]
pub struct ChildProperty {
    pub parent_name: syn::Ident,
    pub property: Property,
}

impl quote::ToTokens for ChildProperty {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.property.0;
        let type_name = quote::format_ident!(
            "{}{}",
            self.parent_name
                .to_string()
                .to_case(convert_case::Case::Pascal),
            self.property
                .0
                .to_string()
                .to_case(convert_case::Case::Pascal),
        );
        match &self.property.1 {
            Value::Boolean(_) => tokens.extend(quote::quote! { #name: bool }),
            Value::String(_) => tokens.extend(quote::quote! { #name: String }),
            Value::Number(_) => tokens.extend(quote::quote! { #name: f64 }),
            Value::InlineTable(_) => tokens.extend(quote::quote! { #name: #type_name  }),
        }
    }
}

#[derive(Clone)]
pub struct ChildPropertyPrelude(pub ChildProperty);

impl quote::ToTokens for ChildPropertyPrelude {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Value::InlineTable(inline_table) = self.0.property.1.clone() {
            ChildInlineTablePrelude(
                inline_table
                    .into_child_inline_table(self.0.property.0.clone(), self.0.parent_name.clone()),
            )
            .to_tokens(tokens);
        }
    }
}

#[derive(Clone)]
pub struct ChildPropertyDefault(pub ChildProperty);

impl std::ops::Deref for ChildPropertyDefault {
    type Target = ChildProperty;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl quote::ToTokens for ChildPropertyDefault {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.property.0.clone();
        let value = ChildValue {
            value: self.property.1.clone(),
            parent_name: self.parent_name.clone(),
            property_name: name.clone(),
        };
        tokens.extend(quote::quote! {
            #name: #value
        });
    }
}
