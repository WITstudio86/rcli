use crate::cli_resolve::{AlphabetType, CodeType};
use anyhow::{Error, Result};
use base64::{engine::general_purpose::URL_SAFE, prelude::*};
pub fn prcocess_base64(
    way: CodeType,
    content: &str,
    alphabet: AlphabetType,
) -> Result<String, Error> {
    match way {
        CodeType::Decode => Ok(decode_base64(content, alphabet)?),
        CodeType::Encode => Ok(encode_base64(content, alphabet)?),
    }
}

fn encode_base64(content: &str, alphabet: AlphabetType) -> Result<String, Error> {
    match alphabet {
        AlphabetType::Standard => {
            let encode = BASE64_STANDARD.encode(content);
            Ok(encode)
        }
        AlphabetType::Safe => {
            let encoded = URL_SAFE.encode(content);
            Ok(encoded)
        }
    }
}

fn decode_base64(content: &str, alphabet: AlphabetType) -> Result<String, Error> {
    match alphabet {
        AlphabetType::Standard => {
            let decoded = BASE64_STANDARD.decode(content.trim())?;
            Ok(String::from_utf8(decoded)?)
        }
        AlphabetType::Safe => {
            let decoded = URL_SAFE.decode(content.trim())?;
            Ok(String::from_utf8(decoded)?)
        }
    }
}
