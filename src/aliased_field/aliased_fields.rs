use syn::{Ident, LitStr};

use super::AliasedField;

#[derive(Debug, Clone)]
pub struct AliasedFields {
    /// The original field's identifier
    pub field_identifiers: Vec<Ident>,

    /// A list of alias names
    pub names_list: Vec<Vec<LitStr>>,

    /// A list of defaultable names
    pub uses_default_selections: Vec<bool>,
}

impl From<Vec<AliasedField>> for AliasedFields {
    fn from(fields: Vec<AliasedField>) -> Self {
        let mut field_identifiers = Vec::new();
        let mut names_list = Vec::new();
        let mut uses_default_selections = Vec::new();

        for AliasedField {
            field_identifier,
            names,
            uses_default
        } in fields
        {
            field_identifiers.push(field_identifier);
            names_list.push(names);
            uses_default_selections.push(uses_default);
        }

        Self {
            field_identifiers,
            names_list,
            uses_default_selections,
        }
    }
}
