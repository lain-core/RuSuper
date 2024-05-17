use std::{
    fmt::{self, Debug},
    thread::current,
};

use crate::emu::VirtualMachine;

use super::TokenSeparators;

/**************************************** Struct and Type definitions ***************************************************/
#[derive(Debug, Clone)]
pub struct InvalidValueError {
    value: String,
}

impl fmt::Display for InvalidValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
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
    fn is_decimal(&self) -> bool;
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

    fn is_decimal(&self) -> bool {
        for char in self.chars() {
            if !char.is_digit(10) {
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

    fn is_decimal(&self) -> bool {
        for char in self.chars() {
            if !char.is_digit(10) {
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

/// Parse a list of directives out into a collection of tokens.
/// Parameters:
///     - `args`:   The input from the user with the command removed and all other values concatenated.
/// Returns:
///     - `Ok(Vec<TokenSeparators>)`:   List of the arguments the user passed parsed into tokens.
///     - `Err(InvalidValueError)`:
pub fn collect_args(args: String) -> Result<Vec<TokenSeparators>, InvalidValueError> {
    let mut delimiters: Vec<TokenSeparators> = vec![];
    let mut value_buffer: String = "".to_string();

    if args.len() > 0 {
        for index in 0..args.len() {
            let current_char = args.chars().nth(index).unwrap();
            match TokenSeparators::from(current_char.to_string().as_str()) {
                // If a separator is found, clear the buffer of other characters and then push on the sepatator.
                TokenSeparators::HexValue => {
                    // println!("Found a Hex delimiter");
                    if value_buffer.len() > 0 {
                        delimiters.push(TokenSeparators::Value(value_buffer));
                        value_buffer = "".to_string();
                    }
                    delimiters.push(TokenSeparators::HexValue);
                    // Everything until the next value is a hex digit.
                }

                TokenSeparators::Offset => {
                    // println!("Found an offset delimiter");
                    if value_buffer.len() > 0 {
                        delimiters.push(TokenSeparators::Value(value_buffer));
                        value_buffer = "".to_string();
                    }
                    delimiters.push(TokenSeparators::Offset)
                }

                // If it is not a delimiting character, push it onto the value buffer.
                TokenSeparators::Invalid => {
                    // println!("Pushing {} onto value", current_char);
                    value_buffer.push(current_char);
                }
                TokenSeparators::Value(_) => (), // This is not a possible option in the TokenSeparators::from constructor
                TokenSeparators::Tag(_) => (), // This is not a possible option in the TokenSeparators::from Constructor
            }
        }
    }
    else {
        return Err(InvalidValueError::from("Length of args passed was 0"));
    }
    // If there's anything left in the value buffer at the end throw it on.
    if value_buffer.len() > 0 {
        delimiters.push(TokenSeparators::Value(value_buffer));
    }

    println!("Final args was {:?}", delimiters);
    return Ok(delimiters);
}

/// Take a composed token list and compute a finalized address value.
/// TODO: This function assumes that there are no tags; That will add complexity.
/// Parameters:
///     - `args`: Arguments passed to the command, as a vector of TokenSeparators.
/// Returns:
///     - `address`: A fully computed address.
pub fn compute_address_from_args(
    args: Vec<TokenSeparators>,
    vm: &VirtualMachine,
) -> Result<usize, InvalidValueError> {
    let mut address: Option<usize> = None;
    let mut modifiers: Vec<TokenSeparators> = vec![];

    for next in args.iter() {
        match next {
            TokenSeparators::HexValue => modifiers.push(TokenSeparators::HexValue),
            TokenSeparators::Offset => modifiers.push(TokenSeparators::Offset),
            TokenSeparators::Value(data) => {
                println!("Found Value: {}", data);
                match apply_modifiers(&mut modifiers, vm, address, data.to_string()) {
                    Ok(newaddr) => address = Some(newaddr),
                    Err(e) => return Err(e),
                }
                // When we find a numeric value, go through the list of modifiers and apply them where necessary.
            }
            TokenSeparators::Tag(name) => {
                println!("Found Tag: {}", name);
            }
            TokenSeparators::Invalid => (),
        }
    }

    match address {
        Some(val) => Ok(val),
        None => Err(InvalidValueError::from(
            "Could not discern a value from arguments passed",
        )),
    }
}

/// Iterate through a vector of TokenSeparators, and for each TokenSeparator::Value, discern if it is a tag or a number.
/// # Parameters:
///     - `tokens`:         A list of tokens to digest.
/// # Returns:
///     - A list of tokens, with TokenSeparator::Values replaced with tags where necessary.
fn collect_tags(tokens: Vec<TokenSeparators>) -> Vec<TokenSeparators> {
    let mut new_vec: Vec<TokenSeparators> = vec![];

    for token in tokens {
        match token {
            TokenSeparators::Value(data) => {
                // If the value is strictly numeric, just push it on as a value.
                if data.is_decimal() || data.is_hex() {
                    new_vec.push(TokenSeparators::Value(data));
                }
                else {
                    // For now, just push it straight on, but in the future we should probably do some more checks(?)
                    new_vec.push(TokenSeparators::Tag(data));
                }
            }
            _ => new_vec.push(token),
        }
    }

    new_vec
}

///
fn apply_modifiers(
    modifiers: &mut Vec<TokenSeparators>,
    vm: &VirtualMachine,
    base_addr: Option<usize>,
    value: String,
) -> Result<usize, InvalidValueError> {
    let mut scratch_value: usize = 0;
    if value.is_decimal() || value.is_hex() {
        // If the number is decimal go ahead and store the decimal representation into the scratch value.
        if value.is_decimal() {
            scratch_value = value.parse::<usize>().unwrap();
        }

        // Digest any modifiers found.
        if modifiers.len() > 0 {
            // Move right to left an apply the modifiers to the value.
            while let Some(modi) = modifiers.pop() {
                // If the value is a hex value, convert it and replace the decimal expression of it.
                if modi == TokenSeparators::HexValue {
                    println!("Applying Hex Modifier to value");
                    scratch_value = value.to_hex()?;
                }
                // If the value is an offset:
                //      - Check if a base address has been set prior to this, and modify it if so.
                //      - Otherwise just apply the offset to the current PC value and store it.
                else if modi == TokenSeparators::Offset {
                    println!("Applying Offset Modifier to value");
                    match base_addr {
                        // If the address is none, the offset is relative to PC.
                        None => {
                            scratch_value = vm.cpu.get_pc() + scratch_value;
                        }
                        Some(addr_value) => {
                            scratch_value = addr_value + scratch_value;
                        }
                    }
                }
            }
        }
    }
    Ok(scratch_value)
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
