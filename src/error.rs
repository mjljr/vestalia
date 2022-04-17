use std::fmt;

#[derive(Debug)]
pub enum Error {
    Api { source: reqwest::Error },
    TextValidation,
    VecValidation,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api { source } => write!(f, "Failed to interact with the Vestaboard API. Re-check the API key pair, host network connectivity, or host DNS configuration. Error: {:#?}", source),
            Self::TextValidation => f.write_str("Invalid characters in text. Vestaboard only supports the following: https://docs.vestaboard.com/characters"),
            Self::VecValidation  => f.write_str("Ensure the characters Vec contains exactly 6 rows and 22 columns of i32."),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}
