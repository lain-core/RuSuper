use std::string;

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
