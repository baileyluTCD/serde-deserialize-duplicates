use syn::Data;

use crate::aliased_field::{AliasedField, AliasedFieldError, AliasedFields};

/// Parse [AliasedFields] from [Data]
pub fn parse_fields(input: Data) -> Result<AliasedFields, AliasedFieldError> {
    let Data::Struct(struct_data) = input else {
        panic!("This macro can only be used on structs")
    };

    Ok(struct_data
        .fields
        .into_iter()
        .map(AliasedField::try_from)
        .collect::<Result<Vec<_>, AliasedFieldError>>()?
        .into())
}
