use syn::{Ident, LitStr};

use super::AliasedField;

#[derive(Debug, Clone)]
pub struct AliasedFields {
    /// The original field's identifier
    pub field_identifiers: Vec<Ident>,

    /// A list of alias names
    pub names: Vec<Vec<LitStr>>,

    /// A list of defaultable names
    pub use_defaults: Vec<bool>,
}

impl From<Vec<AliasedField>> for AliasedFields {
    fn from(fields: Vec<AliasedField>) -> Self {
        let mut field_identifiers_list = Vec::new();
        let mut names_list = Vec::new();
        let mut use_defaults = Vec::new();

        for field in fields {
            let AliasedField {
                field_identifier,
                names,
                use_default,
            } = field;

            field_identifiers_list.push(field_identifier);
            names_list.push(names);
            use_defaults.push(use_default);
        }

        Self {
            field_identifiers: field_identifiers_list,
            names: names_list,
            use_defaults,
        }
    }
}
