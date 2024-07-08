mod aliased_field_error;
mod aliased_fields;

pub use aliased_field_error::AliasedFieldError;
pub use aliased_fields::AliasedFields;

use proc_macro2::{Ident, Span};
use syn::{Field, LitStr};

const SERDE_ATTRIBUTE_PATH: &str = "serde";

const SERDE_FIELD_ATTRIBUTE_PATHS: [&str; 2] = ["alias", "rename"];

/// # Aliased Field
///
/// A field which has been annotated with #[serde(alias = "xyz")] or #[serde(rename = "xyz")]
pub struct AliasedField {
    /// The original field's identifier
    pub field_identifier: Ident,

    /// A list of alias names
    pub names: Vec<LitStr>,

    /// Should this value be deserialized and use the default value on error
    pub uses_default: bool,
}

impl TryFrom<Field> for AliasedField {
    type Error = AliasedFieldError;

    fn try_from(value: Field) -> Result<Self, Self::Error> {
        let field_identifier = value.ident.ok_or(AliasedFieldError::UnnamedField)?;

        let mut names = vec![LitStr::new(
            &field_identifier.to_string(),
            Span::call_site(),
        )];

        let mut uses_default = Default::default();

        value
            .attrs
            .into_iter()
            .filter(|attribute| attribute.path().is_ident(SERDE_ATTRIBUTE_PATH))
            .try_for_each(|attribute| {
                attribute.parse_nested_meta(|meta| {
                    if SERDE_FIELD_ATTRIBUTE_PATHS
                        .iter()
                        .any(|path| meta.path.is_ident(path))
                    {
                        names.push(meta.value()?.parse()?);

                        return Ok(());
                    }

                    if meta.path.is_ident("default") {
                        uses_default = true;

                        return Ok(())
                    }

                    Err(meta.error("Unsupported serde attribute"))
                })
            })?;

        Ok(Self {
            field_identifier,
            names,
            uses_default
        })
    }
}