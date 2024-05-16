use super::VirtualMachine;
use std::process::exit;

const NUM_ADDR_BYTES: usize = 3;

#[derive(Debug, Clone)]
pub struct InvalidValueError {
    value: String,
}

impl InvalidValueError {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.clone().to_owned(),
        }
    }
}

/// Exits the program.
pub fn dbg_exit(_args: Vec<&str>, _vm: &mut VirtualMachine) {
    exit(0);
}

pub fn dbg_help(_args: Vec<&str>, _vm: &mut VirtualMachine) {
    println!("==============================");
    println!("======== RuSuper Help ========\n");
    println!("==============================");
    println!("h, help\n\tOpens this menu");
    println!("exit, quit, q\n\tTerminate the program");
    println!("b $XXXXXX\n\tSets a breakpoint for address $XXXXXX");
    println!("c, r\n\tRun the program until a halt is reached, or a breakpoint is hit");
}

pub fn dbg_invalid(_args: Vec<&str>, _vm: &mut VirtualMachine) {
    dbg_help(_args, _vm);
}

pub fn dbg_continue(_args: Vec<&str>, vm: &mut VirtualMachine) {
    vm.debugger.is_running = true;
}

pub fn dbg_print(args: Vec<&str>, vm: &mut VirtualMachine) {
    let mut token = args.concat();
    if token.contains("+") {
        dbg_print_offset();
    }
    else {
        match string_to_hex(&token) {
            Ok(address) => dbg_print_absolute(address, vm),
            Err(_e) => {
                println!("Error: {} was an invalid address value.", token);
            }
        }
    }
}

fn dbg_print_absolute(address: usize, vm: &mut VirtualMachine) {
    let byte_value = vm.memory.get_byte(address);
    let word_value = vm.memory.get_word(address);

    if byte_value.is_ok() {
        print!(
            "{:#08X}: {:#04X} ",
            address,
            byte_value.expect("Value was error despite checking")
        );
    }
    else {
        print!("Byte Value: Invalid ")
    }

    if word_value.is_ok() {
        print!(
            "Word Value: {:#04X} ",
            word_value.expect("Word value was error despite checking")
        );
    }
    else {
        print!("Word Value: Invalid");
    }

    print!("\n");
}

fn dbg_print_offset() {
    println!("Unimplemented");
}

pub fn string_to_hex(mut value: &str) -> Result<usize, InvalidValueError> {
    if value.starts_with("$") {
        value = value
            .strip_prefix("$")
            .expect("String did not begin with $");
    }
    if value.starts_with("0x") {
        value = value
            .strip_prefix("0x")
            .expect("String did not begin with 0x");
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
        Err(InvalidValueError::new(&format!(
            "Value passed was not a valid hexidecimal number {}",
            value
        )))
    }
}

pub trait IsValidHex {
    fn is_hex(&self) -> bool;
}

impl IsValidHex for String {
    fn is_hex(&self) -> bool {
        for char in self.chars() {
            if !char.is_digit(16) {
                return false;
            }
        }
        return true;
    }
}

impl IsValidHex for &str {
    fn is_hex(&self) -> bool {
        for char in self.chars() {
            if !char.is_digit(16) {
                return false;
            }
        }
        return true;
    }
}
