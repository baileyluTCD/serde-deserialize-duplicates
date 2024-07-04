use proc_macro2::Ident;
use syn::{Data, LitStr};

use crate::aliased_field::AliasedField;

pub fn parse_field_names(input: Data) -> (Vec<Ident>, Vec<Vec<LitStr>>) {
    let Data::Struct(struct_data) = input else {
        panic!("This macro can only be used on structs")
    };

    struct_data
        .fields
        .into_iter()
        .map(AliasedField::try_from)
        .map(Result::unwrap)
        .map(AliasedField::into)
        .collect()
}