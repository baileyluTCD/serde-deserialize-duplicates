//! # Serde Deserialize Duplicates Macros

//! This crate provides utilities for a very specific purpose: _deserializing data with serde with frequent duplicate keys_.
//! It provides two helper macros - [DeserializeFirstDuplicate] and [DeserializeLastDuplicate] to allow a selection of order for this end.
//!
//! ## Using this crate
//! 
//! ### Matching Duplicate Names
//!
//! Take, for example, the following JSON:
//!
//! ```json
//!{
//!     "myNumber": 34,
//!     "myNumber": 67
//!}
//! ```
//!
//! According to the JSON spec, it is perfectly valid JSON data.
//!
//! Sometimes we interface with APIs out of our control which can return data like this, and we simply do not care which value
//! we take, only that we have a value and we do not have a runtime panic.
//!
//! For the following struct, a runtime panic would occur in case the data above was deserialized, due to the duplicate keys.
//! 
//! ```rust
//!#[derive(Serialize, Deserialize)]
//!struct MyStruct {
//!    #[serde(rename = "myNumber")]
//!     my_number: Option<i32>
//!}
//! ```
//! 
//! Thus, this crate provides alternatives to [Deserialize](serde::Deserialize), used as follows:
//! 
//! ```rust
//!#[derive(Serialize, DeserializeFirstDuplicate)]
//!struct MyStruct {
//!    #[serde(rename = "myNumber")]
//!     my_number: Option<i32>
//!}
//! ```
//! 
//! Now, by replacing [Deserialize](serde::Deserialize) with this crate's [DeserializeFirstDuplicate], we can simply use the first instance of "my_number" we encounter and ignore the rest.
//! If you are looking for the last value, see [DeserializeLastDuplicate].
//! 
//! ### Aliased Duplicate Names
//! 
//! This crate also supports deserializing in the case where there is overlapping interpretations of an aliased value.
//! 
//! For example, take this JSON:
//! 
//! ```json
//!{
//!     "breed": "Labarador",
//!     "type": "Labarador"
//!} 
//! ```
//! 
//! Sometimes we have two fields which may or may not exist in JSON that provide identical information and we only want to deserialize one of them in the case we get one.
//! 
//! This can be achieved as follows:
//! ```rust
//!#[derive(Serialize, DeserializeLastDuplicate)]
//!struct Dog {
//!    #[serde(alias = "type")]
//!     breed: Option<String>
//!}
//! ```

#![warn(missing_docs)]

use aliased_field::AliasedField;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

mod aliased_field;
mod generate_deserialization_impl;

use generate_deserialization_impl::generate_deserialization_impl;

/// # Deserialize First Duplicate macro
/// 
/// Macro which creates a deserializer for a struct with named values which takes the first matching value it finds, ignoring the rest.
/// 
/// ## Usage
/// 
///```rust
///use serde_deserialize_duplicates::DeserializeFirstDuplicate;
///use serde::Serialize;
///use serde_json::Result;
///
///// Target struct
///#[derive(Serialize, DeserializeFirstDuplicate)]
///struct ValueHolder {
///    pub value: String
///}
///
///fn main() -> Result<()> {
///    // Mock data with duplicate values
///    let data = r#"
///        {
///            "value": "first",
///            "value": "second",
///        }"#;
///
///    // Deserialize our data into a ValueHolder, taking the first value we find
///    let holder: ValueHolder = serde_json::from_str(data)?;
///
///    assert_eq!(holder.value, "first".to_owned())
///
///    Ok(())
///}
///```
#[proc_macro_derive(DeserializeFirstDuplicate)]
pub fn deserialize_first_duplicate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let Data::Struct(struct_data) = input.data else {
        panic!("This macro can only be used on structs")
    };

    let (field_names, alias_names) = struct_data
        .fields
        .into_iter()
        .map(AliasedField::try_from)
        .map(Result::unwrap)
        .map(AliasedField::into)
        .collect::<(Vec<_>, Vec<_>)>();

    generate_deserialization_impl(
        quote! {
            #(#( #alias_names )|* if #field_names.is_none() => #field_names = Some(map.next_value()?)),*
        },
        struct_name,
        field_names,
    )
}

/// # Deserialize Last Duplicate macro
/// 
/// Macro which creates a deserializer for a struct with named values which takes the last matching value it finds.
/// 
/// ## Usage
/// 
///```rust
///use serde_deserialize_duplicates::DeserializeLastDuplicate;
///use serde::Serialize;
///use serde_json::Result;
///
///// Target struct
///#[derive(Serialize, DeserializeLastDuplicate)]
///struct ValueHolder {
///    pub value: String
///}
///
///fn main() -> Result<()> {
///    // Mock data with duplicate values
///    let data = r#"
///        {
///            "value": "first",
///            "value": "second",
///        }"#;
///
///    // Deserialize our data into a ValueHolder, taking the last value we find
///    let holder: ValueHolder = serde_json::from_str(data)?;
///
///    assert_eq!(holder.value, "second".to_owned())
///
///    Ok(())
///}
///```
#[proc_macro_derive(DeserializeLastDuplicate)]
pub fn deserialize_last_duplicate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let Data::Struct(struct_data) = input.data else {
        panic!("This macro can only be used on structs")
    };

    let (field_names, alias_names) = struct_data
        .fields
        .into_iter()
        .map(AliasedField::try_from)
        .map(Result::unwrap)
        .map(AliasedField::into)
        .collect::<(Vec<_>, Vec<_>)>();

    generate_deserialization_impl(
        quote! {
            #(#( #alias_names )|* => #field_names = Some(map.next_value()?)),*
        },
        struct_name,
        field_names,
    )
}
