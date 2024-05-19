use std::{
    fmt::{self, Debug},
    thread::current,
};

use crate::emu::VirtualMachine;

use super::TokenSeparators;

/**************************************** Struct and Type definitions ***************************************************/

/// Error to generate when a bad argument is passed to the debugger.
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

/// Helpful operators for translating Strings -> Values
pub trait HexOperators {
    fn is_hex(&self) -> bool;
    fn is_decimal(&self) -> bool;
    fn to_hex(&self) -> Result<usize, InvalidValueError>;
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
pub fn collect_args(argvec: Vec<&str>) -> Result<Vec<TokenSeparators>, InvalidValueError> {
    let mut delimiters: Vec<TokenSeparators> = vec![];
    let mut value_buffer: String = String::new();

    let args = argvec.join(" ");
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
                    value_buffer.push(current_char);
                }
                // If we found a divider, flush the value buffer if it has something in it.
                TokenSeparators::Divider => {
                    if value_buffer.len() > 0 {
                        delimiters.push(TokenSeparators::Value(value_buffer));
                        value_buffer = String::new();
                    }
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

    // Lastly, pass it to collect_tags() to pick out what values from tags.
    delimiters = collect_tags(delimiters);

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
            TokenSeparators::Divider => (),
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

/// Take in a list of TokenSeparators, apply the modifiers to the values as desired, and spit out the resultant value.
/// # Parameters:
///     - `modifiers`:      List of TokenSeparators containing the modifiers to apply.
///     - `vm`:             Virtual Machine, in case the resultant value is an offset from the current PC.
///     - `base_addr`:      Base address to operate upon. If `None`, use the PC as the base address.
///     - `value`:          Target value to digest, either a tag or a numeric value.
/// # Returns:
///     - `Ok(value)`:                  The computed address value,
///     - `Err(InvalidValueError)`:     If any of the arguments passed to this function were mangled.
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::iter::zip;

    /**************************************** Test Helpers **************************************************************/

    /// Take two TokenSeparators vectors and check that all the items within are equal
    /// Parameters:
    ///     - `expected_vec`:   Vector of expected arguments.
    ///     - `test_vec`:       Vector of arguments under test.
    fn _arg_vector_is_equal(expected_vec: Vec<TokenSeparators>, test_vec: Vec<TokenSeparators>) {
        if expected_vec.len() != test_vec.len() {
            panic!("Mismatched arguments in test and resultant vector!");
        }
        for index in 0..test_vec.len() {
            assert_eq!(expected_vec[index], test_vec[index]);
        }
    }

    /// Assemble the list of test cases which are driven purely by numeric literals.
    /// Returns:
    ///     - `Vec<TokenSeparators>`: The list of all permutations of token.
    fn assemble_literal_test_cases() -> Vec<Vec<TokenSeparators>> {
        vec![
            // $808000
            // Hex Literal
            vec![
                TokenSeparators::HexValue,
                TokenSeparators::Value(String::from("808000")),
            ],
            // $808000+$0A
            // Hex Literal + Hex Offset
            vec![
                TokenSeparators::HexValue,
                TokenSeparators::Value(String::from("808000")),
                TokenSeparators::Offset,
                TokenSeparators::HexValue,
                TokenSeparators::Value(String::from("0A")),
            ],
            // $808000+50
            // Hex Literal + Decimal Offset
            vec![
                TokenSeparators::HexValue,
                TokenSeparators::Value(String::from("808000")),
                TokenSeparators::Offset,
                TokenSeparators::Value(String::from("50")),
            ],
            // +$0A
            // PC + Hex Offset
            vec![
                TokenSeparators::Offset,
                TokenSeparators::HexValue,
                TokenSeparators::Value(String::from("0A")),
            ],
            // +50
            // PC + Decimal Offset
            vec![
                TokenSeparators::Offset,
                TokenSeparators::Value(String::from("50")),
            ],
            // 50
            // Decimal Literal
            vec![TokenSeparators::Value(String::from("50"))],
            // $50
            // Hex Literal (Duplicate, to cover +50, 50, $50 and see all return diff)
            vec![
                TokenSeparators::HexValue,
                TokenSeparators::Value(String::from("50")),
            ],
            // 50+$0A
            // Decimal Literal + Hex Offset
            vec![
                TokenSeparators::Value(String::from("50")),
                TokenSeparators::Offset,
                TokenSeparators::HexValue,
                TokenSeparators::Value(String::from("0A")),
            ],
            // 50 + 50
            // Decimal Literal + Decimal Offset
            vec![
                TokenSeparators::Value(String::from("50")),
                TokenSeparators::Offset,
                TokenSeparators::Value(String::from("50")),
            ],
        ]
    }

    /// Assemble the list of test cases which are driven by tag values.
    fn assemble_tag_test_cases() -> Vec<Vec<TokenSeparators>> {
        vec![
            // $808000 tag
            // Hex literal followed by tag
            vec![
                TokenSeparators::HexValue,
                TokenSeparators::Value(String::from("808000")),
                TokenSeparators::Tag(String::from("tagname")),
            ],
            // tag $808000
            // Tag followed by Hex Literal
            vec![
                TokenSeparators::Tag(String::from("tagname")),
                TokenSeparators::HexValue,
                TokenSeparators::Value(String::from("808000")),
            ],
            // 50 tag
            // Decimal literal followed by tag
            vec![
                TokenSeparators::Value(String::from("50")),
                TokenSeparators::Tag(String::from("tagname")),
            ],
            // tag 50
            // Tag followed by decimal literal
            vec![
                TokenSeparators::Tag(String::from("tagname")),
                TokenSeparators::Value(String::from("50")),
            ],
            // tag
            // Just a tag name
            vec![TokenSeparators::Tag(String::from("tagname"))],
            // $0A + tag
            // Hex literal offset by tag (Tag should exist prior)
            vec![
                TokenSeparators::HexValue,
                TokenSeparators::Value(String::from("0A")),
                TokenSeparators::Offset,
                TokenSeparators::Tag(String::from("tagname")),
            ],
            // tag + $0A
            // Tag with Hex Offset (Tag should exist prior)
            vec![
                TokenSeparators::Tag(String::from("tagname")),
                TokenSeparators::Offset,
                TokenSeparators::HexValue,
                TokenSeparators::Value(String::from("0A")),
            ],
            // tag + 50
            // Tag with Decimal Offset (Tag should exist prior)
            vec![
                TokenSeparators::Tag(String::from("tagname")),
                TokenSeparators::Offset,
                TokenSeparators::Value(String::from("50")),
            ],
            // 50 + tag
            // Decimal literal with tag offset (Tag should exist prior)
            vec![
                TokenSeparators::Value(String::from("50")),
                TokenSeparators::Offset,
                TokenSeparators::Tag(String::from("tagname")),
            ],
            // tag + tag2
            // 2 tag values (both should exist prior)
            vec![
                TokenSeparators::Tag(String::from("tagname")),
                TokenSeparators::Offset,
                TokenSeparators::Tag(String::from("tagname2")),
            ],
            // tag + tag
            // Adding a tag value to itself (should exist prior)
            vec![
                TokenSeparators::Tag(String::from("tagname")),
                TokenSeparators::Offset,
                TokenSeparators::Tag(String::from("tagname")),
            ],
        ]
    }

    /**************************************** Unit Test Implementations *************************************************/

    mod debugger_literal_tests {
        use super::*;
        #[test]
        fn test_collect_args() {
            let literal_vectors = vec![
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
            let token_vectors = assemble_literal_test_cases();

            for (stringtest, tokentest) in zip(literal_vectors, token_vectors) {
                assert_eq!(collect_args(stringtest).unwrap(), tokentest);
            }
        }

        #[test]
        fn test_compute_address_from_args() {
            let test_vm = VirtualMachine::new();
            let literal_vectors = vec![
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
            let token_vectors = assemble_literal_test_cases();

            for (numerictest, tokentest) in zip(literal_vectors, token_vectors) {
                let test_result = compute_address_from_args(tokentest, &test_vm).unwrap();
                println!(
                    "Expected Result was {:#08X} Test Result was {:#08X}",
                    numerictest, test_result
                );
                assert_eq!(numerictest, test_result);
            }
        }

        #[test]
        fn test_apply_modifiers() {
            let test_vm = VirtualMachine::new();
            let literal_vectors = vec![
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
            let token_vectors = assemble_literal_test_cases();

            for (numerictest, tokentest) in zip(literal_vectors, token_vectors) {
                let test_result = compute_address_from_args(tokentest, &test_vm).unwrap();
                println!(
                    "Expected Result was {:#08X} Test Result was {:#08X}",
                    numerictest, test_result
                );
                assert_eq!(numerictest, test_result);
            }
        }
    }

    mod debugger_tag_tests {
        use super::*;

        #[test]
        fn test_collect_args() {
            let tag_vectors = vec![
                vec!["$808000", "tagname"],
                vec!["tagname", "$808000"],
                vec!["50", "tagname"],
                vec!["tagname", "50"],
                vec!["tagname"],
                vec!["$0A", "+", "tagname"],
                vec!["tagname", "+", "$0A"],
                vec!["tagname", "+", "50"],
                vec!["50", "+", "tagname"],
                vec!["tagname", "+", "tagname2"],
                vec!["tagname", "+", "tagname"],
            ];

            let token_vectors = assemble_tag_test_cases();
            for (literaltest, tokentest) in zip(tag_vectors, token_vectors) {
                let test_result = collect_args(literaltest).unwrap();
                assert_eq!(tokentest, test_result);
            }
        }

        // #[test]
        // fn test_collect_tags() {}

        // #[test]
        // fn test_compute_address_from_args() {}

        // #[test]
        // fn test_apply_modifiers() {}
    }

    #[test]
    fn test_string_to_hex() {
        // Test 100 Random hex values.
        for _step in 0..100 {
            let value: usize = rand::thread_rng().gen_range(0x000000..0xFFFFFF);
            let val_as_string = format!("{:#08X}", value);
            assert_eq!(value, string_to_hex(&val_as_string).unwrap());
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_string_to_hex() {
        string_to_hex("$invalid").unwrap();
    }
}
