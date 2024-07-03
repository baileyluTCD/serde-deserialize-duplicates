use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

/// [AliasedFieldError] describing errors which can occur in associated functions or methods for [AliasedField](super::AliasedField)
#[derive(Debug, Clone)]
pub enum AliasedFieldError {
    /// Occurs if parsing [super::AliasedField] is attempted on a tuple or unit struct
    UnnamedField,

    /// Wrapper for [Errors from the Syn crate](syn::Error)
    SynError(syn::Error),
    
}

impl Display for AliasedFieldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                Self::UnnamedField => "Struct must at least have named fields for duplicated name checking to take place".to_owned(),
                Self::SynError(err) => format!("Error parsing arguments: {}", err)
            }
        )
    }
}

impl Error for AliasedFieldError {}

impl From<syn::Error> for AliasedFieldError {
    fn from(value: syn::Error) -> Self {
        Self::SynError(value)
    }
}