use std::{iter::Map, str::Chars, string};

use super::TokenSeparators;

/**************************************** Struct and Type definitions ***************************************************/
#[derive(Debug, Clone)]
pub struct InvalidValueError {
    value: String,
}

impl From<&str> for InvalidValueError {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string().clone(),
        }
    }
}

impl From<String> for InvalidValueError {
    fn from(value: String) -> Self {
        Self {
            value: value.clone(),
        }
    }
}

pub trait HexOperators {
    fn is_hex(&self) -> bool;
    fn to_hex(&self) -> Result<usize, InvalidValueError>;
}

impl HexOperators for String {
    fn is_hex(&self) -> bool {
        for char in self.chars() {
            if !char.is_digit(16) {
                return false;
            }
        }
        return true;
    }

    fn to_hex(&self) -> Result<usize, InvalidValueError> {
        string_to_hex(self)
    }
}

impl HexOperators for &str {
    fn is_hex(&self) -> bool {
        for char in self.chars() {
            if !char.is_digit(16) {
                return false;
            }
        }
        return true;
    }

    fn to_hex(&self) -> Result<usize, InvalidValueError> {
        string_to_hex(self)
    }
}

/**************************************** File Scope Functions **********************************************************/

pub fn collect_args(args: Vec<&str>) -> Result<Map<TokenSeparators, &str>, InvalidValueError> {
    let token_stream = args.concat();
    let mut delimiters: Vec<TokenSeparators> = vec![];
    let mut value_buffer: String = "".to_string();
    
    for index in 0..token_stream.len() {
        let current_char = token_stream.chars().nth(index).unwrap();
        match TokenSeparators::from(token_stream.get(index..index).unwrap()) {
            // If a separator is found, clear the buffer of other characters and then push on the sepatator.
            TokenSeparators::HexValue => {
                if value_buffer.len() > 0 {
                    delimiters.push(TokenSeparators::Value(value_buffer));
                    value_buffer = "".to_string();
                }
                delimiters.push(TokenSeparators::HexValue);
                // Everything until the next value is a hex digit.
            },
            
            TokenSeparators::Offset => {
                if value_buffer.len() > 0 {
                    delimiters.push(TokenSeparators::Value(value_buffer));
                    value_buffer = "".to_string();
                }
                delimiters.push(TokenSeparators::Offset)
            },
            
            // If it is not a delimiting character, push it onto the value buffer.
            TokenSeparators::Invalid => {
                value_buffer.push(current_char);
            },
            TokenSeparators::Value(_) => () // This is not a possible option in the TokenSeparators::from constructor
        }
    }
    
    Err(InvalidValueError::from(""))
}

/// Convert a String value into a constructed hex value.
/// E.G., "$808000" becomes 0x808000.
/// Parameters:
///     - `text`:   String to parse.
/// Returns:
///     - `Ok(value)`:  Parsed Hex Value.
///     - `Err(e)`:     If string passed contained data other than a marker and hexa-numeric digits.
fn string_to_hex(text: &str) -> Result<usize, InvalidValueError> {
    let mut value = text.to_string().clone();

    if value.starts_with("$") {
        value = value
            .strip_prefix("$")
            .expect("String did not begin with $")
            .to_string();
    }
    if value.starts_with("0x") {
        value = value
            .strip_prefix("0x")
            .expect("String did not begin with 0x")
            .to_string();
    }

    if value.to_string().is_hex() {
        let mut digits: Vec<u32> = vec![];
        // Construct a list of hex digits for each char.
        for char in value.chars() {
            digits.push(
                char.to_digit(16)
                    .expect("Value was not hex despite checking"),
            );
        }

        let mut hex_value = 0;
        let mut digits_iter = digits.iter();
        for iters in 0..digits.len() {
            hex_value |= (digits_iter.next().expect("Iterated past length of digits"))
                << (((digits.len() - 1) - iters) * 4);
        }
        Ok(hex_value as usize)
    }
    else {
        Err(InvalidValueError::from(format!(
            "Value passed was not a valid hexidecimal number {}",
            value
        )))
    }
}

/**************************************** Tests *************************************************************************/
