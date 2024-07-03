mod aliased_field_error;

pub use aliased_field_error::AliasedFieldError;

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
}

impl TryFrom<Field> for AliasedField {
    type Error = AliasedFieldError;

    fn try_from(value: Field) -> Result<Self, Self::Error> {
        let field_identifier = value.ident.ok_or(AliasedFieldError::UnnamedField)?;

        let mut names = vec![LitStr::new(
            &field_identifier.to_string(),
            Span::call_site(),
        )];

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
                    Err(meta.error("Unsupported serde attribute"))
                })
            })?;

        Ok(Self {
            field_identifier,
            names,
        })
    }
}

impl From<AliasedField> for (Ident, Vec<LitStr>) {
    fn from(val: AliasedField) -> Self {
        (val.field_identifier, val.names)
    }
}
