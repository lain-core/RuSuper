use crate::debugger::InvalidDbgArgError;
use std::collections::HashMap;

pub type DebugTagTable = HashMap<String, usize>;

pub trait FindKeyInHashMap {
    fn find_key(&self, value: usize) -> Option<&str>;
}

impl FindKeyInHashMap for DebugTagTable {
    /// Given a value, find the first key that matches in a hashmap.
    fn find_key(&self, value: usize) -> Option<&str> {
        self.iter().find_map(|(key, val)| {
            if *val == value {
                Some(key.as_str())
            } else {
                None
            }
        })
    }
}

/// Allow deleting a value from any Vector of Equatable value <T>.
pub trait RemoveValueFromVector<T: Eq> {
    fn remove_value(&mut self, value: T) -> Option<T>;
}

impl<T: Eq> RemoveValueFromVector<T> for Vec<T> {
    /// Delete value from vector wherever found.
    /// # Parameters:
    ///     - `self`: Vector of type T
    ///     - `value`: Value of type T to scan for.
    /// # Returns:
    ///     - `Some(value)`:    The returned value that was removed from the vector.
    ///     - `None`:           If the value was not present in the vector.
    fn remove_value(&mut self, value: T) -> Option<T> {
        let mut del_index: Vec<usize> = vec![];
        for (index, item) in self.iter_mut().enumerate() {
            if *item == value {
                del_index.push(index);
            }
        }

        if !del_index.is_empty() {
            for index in del_index {
                self.remove(index);
            }
            Some(value)
        } else {
            None
        }
    }
}

/// Helpful operators for translating Strings -> Values
pub trait HexOperators {
    fn is_hex(&self) -> bool;
    fn is_decimal(&self) -> bool;
    fn to_hex(&self) -> Result<usize, InvalidDbgArgError>;
}

impl HexOperators for String {
    /// Check if a value is constructed only from hex digits.
    /// # Parameters:
    ///     - `self`
    /// # Returns:
    ///     - `true`    if the string was a contained hex value,
    ///     - `false`   if the string was not a hex value.
    fn is_hex(&self) -> bool {
        for char in self.chars() {
            if !char.is_digit(16) {
                return false;
            }
        }
        return true;
    }

    /// Check if a value is constructed only from decimal digits.
    /// # Parameters:
    ///     - `self`
    /// # Returns:
    ///     - `true`    if the string consists of only decimal digits.
    ///     - `false`   if the string was not only decimal digits.
    fn is_decimal(&self) -> bool {
        for char in self.chars() {
            if !char.is_digit(10) {
                return false;
            }
        }
        return true;
    }

    fn to_hex(&self) -> Result<usize, InvalidDbgArgError> {
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

    fn is_decimal(&self) -> bool {
        for char in self.chars() {
            if !char.is_digit(10) {
                return false;
            }
        }
        return true;
    }

    fn to_hex(&self) -> Result<usize, InvalidDbgArgError> {
        string_to_hex(self)
    }
}

pub trait HexToString {
    fn to_hex_string(&self) -> String;
}

impl HexToString for usize {
    fn to_hex_string(&self) -> String {
        format!("{:#08X}", self)
            .strip_prefix("0x")
            .unwrap()
            .to_string()
    }
}

/// Convert a String value into a constructed hex value.
/// E.G., "$808000" becomes 0x808000.
/// Parameters:
///     - `text`:   String to parse.
/// Returns:
///     - `Ok(value)`:  Parsed Hex Value.
///     - `Err(e)`:     If string passed contained data other than a marker and hexa-numeric digits.
fn string_to_hex(text: &str) -> Result<usize, InvalidDbgArgError> {
    let mut value = text.to_string().clone();

    if value.starts_with("$") {
        value = String::from(
            value
                .strip_prefix("$")
                .expect("String did not begin with $"),
        );
    }
    if value.starts_with("0x") {
        value = String::from(
            value
                .strip_prefix("0x")
                .expect("String did not begin with 0x"),
        );
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
    } else {
        Err(InvalidDbgArgError::from(format!(
            "Value passed was not a valid hexidecimal number {}",
            value
        )))
    }
}

#[allow(unused_imports)]
mod tests {
    use rand::Rng;

    #[test]
    fn test_string_to_hex() {
        // Test 100 Random hex values.
        // We can test across the entire address range, but it does take a lot longer.
        for _step in 0..100 {
            let value: usize = rand::thread_rng().gen_range(0x000000..0xFFFFFF);
            let val_as_string = format!("{:#08X}", value);
            assert_eq!(value, super::string_to_hex(&val_as_string).unwrap());
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_string_to_hex() {
        super::string_to_hex("$invalid").unwrap();
    }
}
