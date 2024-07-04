use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;

pub fn generate_deserialization_impl(
    key_value_mapping_patterns: proc_macro2::TokenStream,
    deserialization_target_type_identifier: Ident,
    deserialization_target_field_identifiers: Vec<Ident>,
) -> TokenStream {
    quote! {
        impl<'de> serde::Deserialize<'de> for #deserialization_target_type_identifier {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(DuplicateVisitor)
            }
        }

        struct DuplicateVisitor;

        impl<'de> serde::de::Visitor<'de> for DuplicateVisitor {
            type Value = #deserialization_target_type_identifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map with potential duplicate fields")
            }

            fn visit_map<V>(self, mut map: V) -> Result<#deserialization_target_type_identifier, V::Error>
                where
                V: serde::de::MapAccess<'de>,
            {
                #( let mut #deserialization_target_field_identifiers = None; )*

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        #key_value_mapping_patterns ,
                        _ => {
                            let _ = map.next_value::<serde_json::Value>()?;
                        }
                    }
                }

                #( let #deserialization_target_field_identifiers = #deserialization_target_field_identifiers.ok_or_else(|| serde::de::Error::missing_field("id or key"))?; )*

                Ok(#deserialization_target_type_identifier { #(#deserialization_target_field_identifiers),* })
            }
        }
    }
    .into()
}