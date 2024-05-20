use std::fmt::{self, Debug};

use crate::emu::VirtualMachine;

use super::{DebuggerState, TokenSeparator};

/// parser.rs
/// # About this file
/// This file contains the functionality for the debugger to parse arguments from the user into lists of tokens.
/// The general flow is as follows:
///     1. collect_args() takes a set of arguments as a list of strings which are split by spaces, and then encodes them as a list of their token types.
///     2. collect_tags() takes a list of tokens and converts Value() to Tag() where an invalid numeric value is found.
///     3. validate_tag_offsets() takes a list of tokens and converts Tag()s back into Value()s when they already exist.
///     4. compute_address_from_args() takes a list of tokens, and computes the finalized address by calling apply_modifiers().
///         * apply_modifiers() takes a list of tokens and applies all of the modifiers to values where necessary.

/**************************************** Struct and Type definitions ***************************************************/

/// Error to generate when a bad argument is passed to the debugger.
#[derive(Debug, Clone)]
pub struct InvalidDbgArgError {
    value: String,
}

impl fmt::Display for InvalidDbgArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.value) }
}

impl From<&str> for InvalidDbgArgError {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string().clone(),
        }
    }
}

impl From<String> for InvalidDbgArgError {
    fn from(value: String) -> Self {
        Self {
            value: value.clone(),
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

    fn to_hex(&self) -> Result<usize, InvalidDbgArgError> { string_to_hex(self) }
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

    fn to_hex(&self) -> Result<usize, InvalidDbgArgError> { string_to_hex(self) }
}

/**************************************** File Scope Functions **********************************************************/

/// Parse a list of directives out into a collection of tokens.
/// Parameters:
///     - `args`:   The input from the user with the command removed and all other values concatenated.
///     - `debug`:  The debugger with the table of set tags to read.
/// Returns:
///     - `Ok(Vec<TokenSeparator>)`:   List of the arguments the user passed parsed into tokens.
///     - `Err(InvalidDbgArgError)`:
pub fn str_to_args(
    argvec: Vec<&str>, mut debug: &DebuggerState,
) -> Result<Vec<TokenSeparator>, InvalidDbgArgError> {
    let mut arg_result: Result<Vec<TokenSeparator>, InvalidDbgArgError> =
        Err(InvalidDbgArgError::from("Invalid Arguments"));

    // First Pass: Convert the list of strings into a list of tokens as-is.
    if let Ok(delimiters) = collect_args(argvec, &mut debug) {
        // Second Pass: Convert all of the TokenSeparator::Value()s which correspond to tags into tags
        arg_result = Ok(collect_tags(delimiters));

        // Third Pass: Look through all of the tags, check if they already exist and convert them back into values where possible.
        //      Any remaining tags will have to be new tags.
        match validate_tag_offsets(arg_result.unwrap(), debug) {
            Ok(new_list) => {
                arg_result = Ok(new_list);
            }
            Err(e) => arg_result = Err(e),
        }
    }

    return arg_result;
}

/// Take a composed token list and compute a finalized address value.
/// TODO: This function assumes that there are no tags; That will add complexity.
/// Parameters:
///     - `args`: Arguments passed to the command, as a vector of TokenSeparator.
///     - `vm`:     Virtual machine to access memory or program counter from.
/// Returns:
///     - `address`: A fully computed address.
pub fn compute_address_from_args(
    args: Vec<TokenSeparator>, vm: &VirtualMachine,
) -> Result<usize, InvalidDbgArgError> {
    let mut address: Option<usize> = None;
    let mut modifiers: Vec<TokenSeparator> = vec![];

    for next in args.iter() {
        match next {
            TokenSeparator::HexValue => modifiers.push(TokenSeparator::HexValue),
            TokenSeparator::Offset => modifiers.push(TokenSeparator::Offset),
            TokenSeparator::Value(data) => {
                println!("Found Value: {}", data);
                match apply_modifiers(&mut modifiers, vm, address, data.to_string()) {
                    Ok(newaddr) => address = Some(newaddr),
                    Err(e) => return Err(e),
                }
                // When we find a numeric value, go through the list of modifiers and apply them where necessary.
            }
            TokenSeparator::Tag(name) => {
                println!("Found Tag: {}", name);
            }
            TokenSeparator::Invalid => (),
            TokenSeparator::Divider => (),
        }
    }

    match address {
        Some(val) => Ok(val),
        None => Err(InvalidDbgArgError::from(
            "Could not discern a value from arguments passed",
        )),
    }
}

fn collect_args(
    argvec: Vec<&str>, mut debug: &DebuggerState,
) -> Result<Vec<TokenSeparator>, InvalidDbgArgError> {
    let mut delimiters: Vec<TokenSeparator> = vec![];
    let mut value_buffer: String = String::new();

    println!("Input was {:?}", &argvec);

    let args = argvec.join(" ");
    if args.len() > 0 {
        for index in 0..args.len() {
            let current_char = args.chars().nth(index).unwrap();
            match TokenSeparator::from(current_char.to_string().as_str()) {
                // If a separator is found, clear the buffer of other characters and then push on the sepatator.
                TokenSeparator::HexValue => {
                    if value_buffer.len() > 0 {
                        delimiters.push(TokenSeparator::Value(value_buffer));
                        value_buffer = String::new();
                    }
                    delimiters.push(TokenSeparator::HexValue);
                }

                TokenSeparator::Offset => {
                    if value_buffer.len() > 0 {
                        delimiters.push(TokenSeparator::Value(value_buffer));
                        value_buffer = String::new();
                    }
                    delimiters.push(TokenSeparator::Offset)
                }

                // If it is not a delimiting character, push it onto the value buffer.
                TokenSeparator::Invalid => {
                    value_buffer.push(current_char);
                }
                // If we found a divider, flush the value buffer if it has something in it.
                TokenSeparator::Divider => {
                    if value_buffer.len() > 0 {
                        delimiters.push(TokenSeparator::Value(value_buffer));
                        value_buffer = String::new();
                    }
                }
                TokenSeparator::Value(_) => (), // This is not a possible option in the TokenSeparator::from constructor
                TokenSeparator::Tag(_) => (), // This is not a possible option in the TokenSeparator::from Constructor
            }
        }
    }
    else {
        return Err(InvalidDbgArgError::from("Length of args passed was 0"));
    }
    // If there's anything left in the value buffer at the end throw it on.
    if value_buffer.len() > 0 {
        delimiters.push(TokenSeparator::Value(value_buffer));
    }
    Ok(delimiters)
}

/// Iterate through a vector of TokenSeparator, and for each TokenSeparator::Value, discern if it is a tag or a number.
/// # Parameters:
///     - `tokens`:         A list of tokens to digest.
/// # Returns:
///     - A list of tokens, with TokenSeparator::Values replaced with tags where necessary.
fn collect_tags(tokens: Vec<TokenSeparator>) -> Vec<TokenSeparator> {
    let mut new_vec: Vec<TokenSeparator> = vec![];

    for token in tokens {
        match token {
            TokenSeparator::Value(data) => {
                // If the value is strictly numeric, just push it on as a value.
                if data.is_decimal() || data.is_hex() {
                    new_vec.push(TokenSeparator::Value(data));
                }
                else {
                    // For now, just push it straight on, but in the future we should probably do some more checks(?)
                    new_vec.push(TokenSeparator::Tag(data));
                }
            }
            _ => new_vec.push(token),
        }
    }

    new_vec
}

/// Parse a full list of arguments and check that if any tags within are being modified by an offset.
///     If they are, they must exist at the time we are checking, or else we have been handed a bad argument.
/// Parameters:
///     - `tokens`:     Mutable list of TokenSeparator tokens to parse for tags and offsets.
///     - `debug`:      Debugger with the list of tags to match from.
/// Returns:
///     - `Ok(Vec<TokenSeparator>)`:    A new list of tokens, with the tags that could be found replaced by absolute values where possible.
///     - `Err(InvalidDbgArgError)`:    An error informing the user the first tag which is invalid.
fn validate_tag_offsets(
    mut tokens: Vec<TokenSeparator>, debug: &DebuggerState,
) -> Result<Vec<TokenSeparator>, InvalidDbgArgError> {
    let mut last_token: Option<TokenSeparator> = None;
    let mut curr_token: Option<TokenSeparator> = tokens.pop();
    let mut new_tokens: Vec<TokenSeparator> = vec![];

    println!(
        "Values passed in to validate_tag_offsets was: {:?}",
        &tokens
    );

    while curr_token != None {
        match curr_token {
            // If the current token is an offset, check if it is following a Tag.
            Some(TokenSeparator::Offset) => {
                // If the last operator was a tag, then the tag must be derefable, or we can throw an error.
                // If the last operator was NOT a tag, just push this back on.
                if let Some(TokenSeparator::Tag(tagname)) = last_token {
                    if let None = debug.tags.get(&tagname) {
                        return Err(InvalidDbgArgError::from(format!(
                            "Attempted to compute an offset from a tag which does not exist: {}",
                            &tagname
                        )));
                    }
                    else {
                        new_tokens.push(TokenSeparator::Offset);
                    }
                }
                else {
                    new_tokens.push(TokenSeparator::Offset);
                }
            }
            Some(TokenSeparator::Tag(ref tagname)) => {
                // If the last value was an offset, then this tag MUST be derefable, or we can throw an error.
                // If the last value was NOT an offset, then this could be a new tag.
                if let Some(tagvalue) = debug.tags.get(tagname) {
                    new_tokens.push(TokenSeparator::Value(format!("{:08X}", *tagvalue)));
                }
                else {
                    if let Some(TokenSeparator::Offset) = last_token {
                        return Err(InvalidDbgArgError::from(format!(
                            "Attempted to compute an offset from a tag which does not exist: {}",
                            tagname
                        )));
                    }
                    // Otherwise, this is potentially a new tag, so just push it back on as a tag, and let someone else manage adding it to the list.
                    else {
                        new_tokens.push(TokenSeparator::Tag(tagname.clone()))
                    }
                }
            }
            Some(ref token) => new_tokens.push(token.clone()),
            None => (),
        }
        last_token = curr_token.clone();
        curr_token = tokens.pop();
    }


    println!("Outcome after validate_tag_offsets was {:?}", &new_tokens);

    Ok(new_tokens)
}

/// Take in a list of TokenSeparator, apply the modifiers to the values as desired, and spit out the resultant value.
/// # Parameters:
///     - `modifiers`:      List of TokenSeparator containing the modifiers to apply.
///     - `vm`:             Virtual Machine, in case the resultant value is an offset from the current PC.
///     - `base_addr`:      Base address to operate upon. If `None`, use the PC as the base address.
///     - `value`:          Target value to digest, either a tag or a numeric value.
/// # Returns:
///     - `Ok(value)`:                  The computed address value,
///     - `Err(InvalidDbgArgError)`:     If any of the arguments passed to this function were mangled.
fn apply_modifiers(
    modifiers: &mut Vec<TokenSeparator>, vm: &VirtualMachine, base_addr: Option<usize>,
    value: String,
) -> Result<usize, InvalidDbgArgError> {
    let mut scratch_value: usize = 0;
    if value.is_decimal() || value.is_hex() {
        // If the number is decimal go ahead and store the decimal representation into the scratch value.
        if value.is_decimal() {
            scratch_value = value.parse::<usize>().unwrap();
        }

        // Digest any modifiers found.
        if modifiers.len() > 0 {
            // Move right to left an apply the modifiers to the value.
            while let Some(ref modi) = modifiers.pop() {
                // If the value is a hex value, convert it and replace the decimal expression of it.
                if *modi == TokenSeparator::HexValue {
                    println!("Applying Hex Modifier to value");
                    scratch_value = value.to_hex()?;
                }
                // If the value is an offset:
                //      - Check if a base address has been set prior to this, and modify it if so.
                //      - Otherwise just apply the offset to the current PC value and store it.
                else if *modi == TokenSeparator::Offset {
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
    }
    else {
        Err(InvalidDbgArgError::from(format!(
            "Value passed was not a valid hexidecimal number {}",
            value
        )))
    }
}

/**************************************** Tests *************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::iter::zip;
    use testconst::*;

    /**************************************** Test Helpers **************************************************************/

    mod testconst {
        use super::*;

        pub const TEST_TAG_NAME: &str = "tagname";
        pub const TEST_TAG_NAME2: &str = "tagname2";

        /// Test results for the outcome of each of the tests performed.

        /// The representation of the arguments passed as a vector of strings.

        /// Test cases which are driven purely by numeric literals.
        pub fn token_literal_test_cases() -> Vec<Vec<TokenSeparator>> {
            vec![
                // $808000
                // Hex Literal
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                ],
                // $808000+$0A
                // Hex Literal + Hex Offset
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                ],
                // $808000+50
                // Hex Literal + Decimal Offset
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                ],
                // +$0A
                // PC + Hex Offset
                vec![
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                ],
                // +50
                // PC + Decimal Offset
                vec![
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                ],
                // 50
                // Decimal Literal
                vec![TokenSeparator::Value(String::from("50"))],
                // $50
                // Hex Literal (Duplicate, to cover +50, 50, $50 and see all return diff)
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("50")),
                ],
                // 50+$0A
                // Decimal Literal + Hex Offset
                vec![
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                ],
                // 50 + 50
                // Decimal Literal + Decimal Offset
                vec![
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                ],
            ]
        }

        /// Test cases where a tag is represented as Value(tagname).
        pub fn token_tag_as_value() -> Vec<Vec<TokenSeparator>> {
            vec![
                /****** Tag before value ******/
                // tag $808000
                vec![
                    TokenSeparator::Value(String::from("tagname")),
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                ],
                // tag 50
                vec![
                    TokenSeparator::Value(String::from("tagname")),
                    TokenSeparator::Value(String::from("50")),
                ],
                // tag $808000 + $0A
                vec![
                    TokenSeparator::Value(String::from("tagname")),
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                ],
                // tag $808000 + 50
                vec![
                    TokenSeparator::Value(String::from("tagname")),
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                ],
                // tag +$0A
                vec![
                    TokenSeparator::Value(String::from("tagname")),
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                ],
                // tag +50
                vec![
                    TokenSeparator::Value(String::from("tagname")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                ],
                /****** Value before tag ******/
                // $808000 tag
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Value(String::from("tagname")),
                ],
                // 50 tag
                vec![
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Value(String::from("tagname")),
                ],
                // $808000 + $0A tag
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                    TokenSeparator::Value(String::from("tagname")),
                ],
                // $808000 + 50 tag
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Value(String::from("tagname")),
                ],
                // $0A + tag
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("tagname")),
                ],
                // 50 + tag
                vec![
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("tagname")),
                ],
                // +$0A tag
                vec![
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                    TokenSeparator::Value(String::from("tagname")),
                ],
                // +50 tag
                vec![
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Value(String::from("tagname")),
                ],
                /****** Other tag configurations ******/
                // tag
                vec![TokenSeparator::Value(String::from("tagname"))],
                // tag + tag
                vec![
                    TokenSeparator::Value(String::from("tagname")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("tagname")),
                ],
                // tag + tag2
                vec![
                    TokenSeparator::Value(String::from("tagname")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("tagname2")),
                ],
            ]
        }

        /// Test cases where a tag is represented as Tag(tagname).
        pub fn token_tag_as_tags() -> Vec<Vec<TokenSeparator>> {
            vec![
                /******* Value After Tag *******/
                // tag $808000
                // Tag followed by Hex Literal
                vec![
                    TokenSeparator::Tag(String::from("tagname")),
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                ],
                // tag 50
                // Tag followed by decimal literal
                vec![
                    TokenSeparator::Tag(String::from("tagname")),
                    TokenSeparator::Value(String::from("50")),
                ],
                // tag $808000 + $0A
                // Tag, as specified by a hex literal + hex offset
                vec![
                    TokenSeparator::Tag(String::from("tagname")),
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                ],
                // tag $808000 + 50
                // Tag, as specified by a hex literal + decimal offset
                vec![
                    TokenSeparator::Tag(String::from("tagname")),
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                ],
                // tag + $0A
                // Tag with Hex Offset (Tag should exist prior)
                vec![
                    TokenSeparator::Tag(String::from("tagname")),
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                ],
                // tag + 50
                // Tag with Decimal Offset (Tag should exist prior)
                vec![
                    TokenSeparator::Tag(String::from("tagname")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                ],
                /******* Value before Tag *******/
                // $808000 tag
                // Hex literal followed by tag
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Tag(String::from("tagname")),
                ],
                // 50 tag
                // Decimal literal followed by tag
                vec![
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Tag(String::from("tagname")),
                ],
                // $808000 + $0A tag
                // tag, as defined as Hex literal + Hex offset
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                    TokenSeparator::Tag(String::from("tagname")),
                ],
                // $808000 + 50 tag
                // tag, as defined as Hex Literal + Decimal Offset
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Tag(String::from("tagname")),
                ],
                // $0A + tag
                // Hex literal offset by tag (Tag should exist prior)
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                    TokenSeparator::Offset,
                    TokenSeparator::Tag(String::from("tagname")),
                ],
                // 50 + tag
                // Decimal literal with tag offset (Tag should exist prior)
                vec![
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Offset,
                    TokenSeparator::Tag(String::from("tagname")),
                ],
                // +$0A tag
                // tag as Hex offset from PC
                vec![
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                    TokenSeparator::Tag(String::from("tagname")),
                ],
                // +50 tag
                // tag as decimal offset from PC
                vec![
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Tag(String::from("tagname")),
                ],
                /******* Other Tag Cases *******/
                // tag
                // Just a tag name
                vec![TokenSeparator::Tag(String::from("tagname"))],
                // tag + tag
                // Adding a tag value to itself (should exist prior)
                vec![
                    TokenSeparator::Tag(String::from("tagname")),
                    TokenSeparator::Offset,
                    TokenSeparator::Tag(String::from("tagname")),
                ],
                // tag + tag2
                // 2 tag values (both should exist prior)
                vec![
                    TokenSeparator::Tag(String::from("tagname")),
                    TokenSeparator::Offset,
                    TokenSeparator::Tag(String::from("tagname2")),
                ],
            ]
        }

        /// Test cases where a tag is represented as the value at tagname's address.
        pub fn token_tag_as_deref() -> Vec<Vec<TokenSeparator>> {
            vec![
                /******* Value After Tag *******/
                // tag $808000
                // Tag followed by Hex Literal
                vec![
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                ],
                // tag 50
                // Tag followed by decimal literal
                vec![
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Value(String::from("50")),
                ],
                // tag $808000 + $0A
                // Tag, as specified by a hex literal + hex offset
                vec![
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                ],
                // tag $808000 + 50
                // Tag, as specified by a hex literal + decimal offset
                vec![
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                ],
                // tag + $0A
                // Tag with Hex Offset (Tag should exist prior)
                vec![
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                ],
                // tag + 50
                // Tag with Decimal Offset (Tag should exist prior)
                vec![
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                ],
                /******* Value before Tag *******/
                // $808000 tag
                // Hex literal followed by tag
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Value(String::from("808000")),
                ],
                // 50 tag
                // Decimal literal followed by tag
                vec![
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Value(String::from("808000")),
                ],
                // $808000 + $0A tag
                // tag, as defined as Hex literal + Hex offset
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                    TokenSeparator::Value(String::from("808000")),
                ],
                // $808000 + 50 tag
                // tag, as defined as Hex Literal + Decimal Offset
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Value(String::from("808000")),
                ],
                // $0A + tag
                // Hex literal offset by tag (Tag should exist prior)
                vec![
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("808000")),
                ],
                // 50 + tag
                // Decimal literal with tag offset (Tag should exist prior)
                vec![
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("808000")),
                ],
                // +$0A tag
                // tag as Hex offset from PC
                vec![
                    TokenSeparator::Offset,
                    TokenSeparator::HexValue,
                    TokenSeparator::Value(String::from("0A")),
                    TokenSeparator::Value(String::from("808000")),
                ],
                // +50 tag
                // tag as decimal offset from PC
                vec![
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("50")),
                    TokenSeparator::Value(String::from("808000")),
                ],
                /******* Other Tag Cases *******/
                // tag
                // Just a tag name
                vec![TokenSeparator::Value(String::from("808000"))],
                // tag + tag
                // Adding a tag value to itself (should exist prior)
                vec![
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("808000")),
                ],
                // tag + tag2
                // 2 tag values (both should exist prior)
                vec![
                    TokenSeparator::Value(String::from("808000")),
                    TokenSeparator::Offset,
                    TokenSeparator::Value(String::from("0A")),
                ],
            ]
        }
    }

    /**************************************** Unit Test Implementations *************************************************/

    mod debugger_literal_tests {
        use super::testconst::*;
        use super::*;

        #[test]
        fn test_collect_args() {
            let test_debugger = DebuggerState::new();
            let string_vectors: Vec<Vec<&str>> = vec![
                vec!["$808000"],
                vec!["$808000", "+", "$0A"],
                vec!["$808000", "+", "50"],
                vec!["+", "$0A"],
                vec!["+", "50"],
                vec!["50"],
                vec!["$50"],
                vec!["50", "+", "$0A"],
                vec!["50", "+", "50"],
            ];
            let token_vectors = token_literal_test_cases();

            for (test_input, expected_result) in zip(string_vectors, token_vectors) {
                assert_eq!(
                    expected_result,
                    collect_args(test_input, &test_debugger).unwrap()
                );
            }
        }

        #[test]
        fn test_compute_address_from_args() {
            let test_vm = VirtualMachine::new();
            let numeric_result_vector: Vec<usize> = vec![
                0x808000, // $808000
                0x80800A, // $808000+$0A
                0x808032, // $808000+50
                0x80800A, // +$0A
                0x808032, // +50
                0x000032, // 50
                0x000050, // $50
                0x00003C, // 50+$0A
                0x000064, // 50+50
            ];
            let token_vectors = token_literal_test_cases();

            for (test_input, expected_result) in zip(token_vectors, numeric_result_vector) {
                let test_result = compute_address_from_args(test_input, &test_vm).unwrap();
                println!(
                    "Expected Result was {:#08X} Test Result was {:#08X}",
                    expected_result, test_result
                );
                assert_eq!(expected_result, test_result);
            }
        }

        #[test]
        fn test_apply_modifiers() {
            let test_vm = VirtualMachine::new();
            let token_vectors = token_literal_test_cases();
            let numeric_result_vector: Vec<usize> = vec![
                0x808000, // $808000
                0x80800A, // $808000+$0A
                0x808032, // $808000+50
                0x80800A, // +$0A
                0x808032, // +50
                0x000032, // 50
                0x000050, // $50
                0x00003C, // 50+$0A
                0x000064, // 50+50
            ];

            for (test_input, expected_result) in zip(token_vectors, numeric_result_vector) {
                let test_result = compute_address_from_args(test_input, &test_vm).unwrap();
                println!(
                    "Expected Result was {:#08X} Test Result was {:#08X}",
                    expected_result, test_result
                );
                assert_eq!(expected_result, test_result);
            }
        }
    }

    mod debugger_tag_tests {
        use super::testconst::*;
        use super::*;

        #[test]
        fn test_collect_args() {
            let test_debugger = DebuggerState::new();
            let string_vectors: Vec<Vec<&str>> = vec![
                /****** Tag before value ******/
                vec!["tagname", "$808000"],
                vec!["tagname", "50"],
                vec!["tagname", "$808000", "+", "$0A"],
                vec!["tagname", "$808000", "+", "50"],
                vec!["tagname", "+", "$0A"],
                vec!["tagname", "+", "50"],
                /****** Value before tag ******/
                vec!["$808000", "tagname"],
                vec!["50", "tagname"],
                vec!["$808000", "+", "$0A", "tagname"],
                vec!["$808000", "+", "50", "tagname"],
                vec!["$0A", "+", "tagname"],
                vec!["50", "+", "tagname"],
                vec!["+", "$0A", "tagname"],
                vec!["+", "50", "tagname"],
                /****** Other Combinations ******/
                vec!["tagname"],
                vec!["tagname", "+", "tagname"],
                vec!["tagname", "+", "tagname2"],
            ];
            let token_vectors = token_tag_as_value();
            for (test_input, expected_result) in zip(string_vectors, token_vectors) {
                let test_result = collect_args(test_input, &test_debugger).unwrap();
                assert_eq!(expected_result, test_result);
            }
        }

        #[test]
        fn test_collect_tags() {
            // Get the table of test cases where a tag is represented as TokenSeparator::Value(tagname).
            let test_token_vectors = token_tag_as_value();

            // Get the table of test cases where a tag is represented as a TokenSeparator::Tag(tagname).
            let expected_token_vectors = token_tag_as_tags();

            for (test_input, expected_result) in zip(test_token_vectors, expected_token_vectors) {
                let test_result = collect_tags(test_input);
                assert_eq!(expected_result, test_result);
            }
        }

        #[test]
        fn test_validate_tag_offsets() {
            let mut test_debugger = DebuggerState::new();
            test_debugger
                .tags
                .insert(String::from(TEST_TAG_NAME), 0x808000);
            test_debugger
                .tags
                .insert(String::from(TEST_TAG_NAME2), 0x0A);
            let test_tag_tokens = token_tag_as_tags();
            let test_deref_tokens = token_tag_as_deref();

            for (test_input, expected_result) in zip(test_tag_tokens, test_deref_tokens) {
                let test_result = validate_tag_offsets(test_input, &test_debugger).unwrap();
                assert_eq!(expected_result, test_result);
            }
        }

        // #[test]
        // fn test_compute_address_from_args() {}

        // #[test]
        // fn test_apply_modifiers() {}
    }

    #[test]
    fn test_string_to_hex() {
        // Test 100 Random hex values.
        // We can test across the entire address range, but it does take a lot longer.
        for _step in 0..100 {
            let value: usize = rand::thread_rng().gen_range(0x000000..0xFFFFFF);
            let val_as_string = format!("{:#08X}", value);
            assert_eq!(value, string_to_hex(&val_as_string).unwrap());
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_string_to_hex() { string_to_hex("$invalid").unwrap(); }
}
