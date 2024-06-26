use crate::debugger::{
    utils::{DebugTagTable, FindKeyInHashMap, RemoveValueFromVector},
    InvalidDbgArgError,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParserData {
    // TODO: Might be nice for this to instead be a Map<usize, (usize, String)> so we can show the
    // user what they entered as a formula rather than only the resultant address.
    addresses: Vec<usize>,
    // TODO: If I was a bit slicker, this could be a HashMap<String, &[u8]> maybe?
    tag_db: DebugTagTable,
}

impl ParserData {
    pub fn new() -> Self {
        Self {
            addresses: vec![],
            tag_db: HashMap::new(),
        }
    }

    /// Look to see if the tag table contains a tag, and return it's value if so.
    /// # Parameters:
    ///     - `self`
    ///     - `tag`: Tag name to dereference in the table.
    /// # Returns:
    ///     - `Some(&usize)`:   Address existing at that tag.
    ///     - `None`:           If no value exists for that tag name.
    pub fn get_tag(&self, tag: &str) -> Option<&usize> { self.tag_db.get(tag) }

    /// Look to see if the table contains a tag, and return it's name if so.
    /// # Parameters:
    ///     - `self`
    ///     - `address`: Address to try to find a tag name at.
    /// # Returns:
    ///     - `Some(tagname)`:  The name of the tag, if found.
    ///     - `None`:           If the tag was not present in the list.
    pub fn get_tag_name(&self, address: usize) -> Option<&str> { self.tag_db.find_key(address) }

    /// Wrapper for inserting a breakpoint with a tag and an address.
    /// If a tag is already present, updates the value and returns it, Otherwise, inserts into the
    /// table.
    ///
    /// This function cannot fail.
    ///
    /// # Parameters:
    ///     - `&mut self`
    ///     - `tag`: Tag value to insert into the table.
    ///     - `address`: Associated value to insert into the table.
    /// # Returns:
    ///     - `Some(address)`:  The updated address of the inserted tag,
    ///     - `None`:           If the tag was new
    pub fn insert_tag(&mut self, tag: &str, address: usize) -> Option<usize> {
        if let Some(old_value) = self.tag_db.insert(String::from(tag), address) {
            self.addresses.remove_value(old_value);
            self.addresses.push(address);
            Some(address)
        }
        else {
            self.addresses.push(address);
            None
        }
    }

    /// Delete a tag and it's associated address from the hash map and the table.
    /// # Parameters:
    ///     - `&mut self`
    ///     - `tag`: Tag name to delete.
    /// # Returns:
    ///     - `Some(tagname, value)`: If the value was present.
    ///     - `None`:                   If the value didn't exist in the table.
    pub fn delete_tag(&mut self, tag: &str) -> Option<(String, usize)> {
        if let Some(value) = self.tag_db.remove_entry(tag) {
            self.delete(value.1);
            return Some(value);
        }
        None
    }

    /// Check if a value is present in the breakpoint table.
    /// # Parameters:
    ///     - `self`
    ///     - `address`: Address to test.
    /// # Returns:
    ///     - `Some(&usize)`:   The address that was passed in, which was found.
    ///     - `None`:           If the address was not already present.
    pub fn get(&self, address: usize) -> Option<usize> {
        if self.addresses.contains(&address) {
            Some(address)
        }
        else {
            None
        }
    }

    /// Attempt to insert a value alone.
    /// # Parameters:
    ///     - `&mut self`
    ///     - `address`: Address to insert.
    /// # Returns:
    ///     - `Ok(())`: If the value did not exist in the table.
    ///     - `Err(tagname)`: If the value already existed, and is named.
    ///     - `Err(value)`: If the value already existed in the table, but didn't have an
    ///     associated name.
    pub fn insert(&mut self, address: usize) -> Result<(), InvalidDbgArgError> {
        if let Some(value) = self.get(address) {
            if let Some(name) = self.tag_db.find_key(address) {
                Err(InvalidDbgArgError::from(format!(
                    "Breakpoint {} already exists at {:#08X}",
                    name, value
                )))
            }
            else {
                Err(InvalidDbgArgError::from(format!(
                    "A breakpoint was already set at {:#08X}",
                    value
                )))
            }
        }
        else {
            self.addresses.push(address);
            Ok(())
        }
    }

    /// Wrapper for utils::remove_value.
    /// # Parameters:
    ///     - `&mut self`
    ///     - `address`: The value to remove from the breakpoint list.
    /// # Returns:
    ///     - `Some(address)`:  The address that was removed from the table
    ///     - `None`:           If the address was not present in the table.
    pub fn delete(&mut self, address: usize) -> Option<usize> {
        if self.addresses.remove_value(address) {
            Some(address)
        }
        else {
            None
        }
    }

    /// Display the contents of this instance.
    pub fn display(&self) {
        println!();
        println!("  Address  | Tag Name  ");
        println!("-----------------------");
        // TODO: Better way to do this?

        for value in self.addresses.iter() {
            print!("  ");
            print!("{:#08X} |", value);
            if let Some(name) = self.get_tag_name(*value) {
                print!(" {}", name);
            }
            println!();
        }
        println!("  \n-----------------------");
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::debugger::parser::tests::testconst::{
        self, TEST_BASE_ADDR, TEST_HEX_VALUE, TEST_TAG_NAME,
    };

    use super::*;

    #[test]
    fn test_parser_data_insert() {
        let expected_numerics = testconst::literal_numeric_results();

        let mut test_parser_data = ParserData::new();

        for expected_result in expected_numerics {
            assert!(test_parser_data.insert(expected_result).is_ok());
            test_parser_data = ParserData::new();
        }
    }

    #[test]
    fn test_parser_data_get() {
        let expected_numerics = testconst::literal_numeric_results();
        let mut test_parser_data = ParserData::new();

        for expected_result in expected_numerics {
            // Manually push the value into the list so we are only testing get()
            test_parser_data.addresses.push(expected_result);

            assert_eq!(
                test_parser_data.get(expected_result).unwrap(),
                expected_result
            );

            // Construct a new ParserData instead of deleting the entries so we are only testing
            // get()
            test_parser_data = ParserData::new();
        }
    }

    #[test]
    fn test_parser_data_delete() {
        let expected_numerics = testconst::literal_numeric_results();
        let mut test_parser_data = ParserData::new();

        for expected_result in expected_numerics {
            test_parser_data.addresses.push(expected_result);
            assert_eq!(
                test_parser_data.delete(expected_result).unwrap(),
                expected_result
            );
            assert!(!test_parser_data.addresses.contains(&expected_result));
        }
    }

    #[test]
    fn test_parser_data_tag_insert() {
        let mut test_parser_data = ParserData::new();

        // Test that the insert works and returns None.
        assert!(test_parser_data
            .insert_tag(TEST_TAG_NAME, TEST_BASE_ADDR)
            .is_none());

        // Check the value is correct.
        assert_eq!(test_parser_data.tag_db[TEST_TAG_NAME], TEST_BASE_ADDR);

        // Test that an update works and returns Some(oldvalue).
        assert_eq!(
            test_parser_data.insert_tag(TEST_TAG_NAME, TEST_BASE_ADDR + TEST_HEX_VALUE),
            Some(TEST_BASE_ADDR + TEST_HEX_VALUE)
        );

        // Check that the new value is correct.
        assert_eq!(
            test_parser_data.tag_db[TEST_TAG_NAME],
            TEST_BASE_ADDR + TEST_HEX_VALUE
        );
    }

    #[test]
    fn test_parser_data_tag_get() {
        let mut test_parser_data = ParserData::new();

        test_parser_data
            .tag_db
            .insert(String::from(TEST_TAG_NAME), TEST_BASE_ADDR);

        // Check that the value is correct.
        assert_eq!(
            *(test_parser_data.get_tag(TEST_TAG_NAME).unwrap()),
            TEST_BASE_ADDR
        );

        test_parser_data
            .tag_db
            .insert(String::from(TEST_TAG_NAME), TEST_BASE_ADDR + TEST_HEX_VALUE);

        // Check that the updated value is correct.
        assert_eq!(
            *(test_parser_data.get_tag(TEST_TAG_NAME).unwrap()),
            TEST_BASE_ADDR + TEST_HEX_VALUE
        );
    }

    #[test]
    fn test_parser_data_tag_delete() {
        let mut test_parser_data = ParserData::new();

        test_parser_data
            .tag_db
            .insert(String::from(TEST_TAG_NAME), TEST_BASE_ADDR);

        // Check that the tag has been deleted correctly and the returned value is correct.
        assert_eq!(
            test_parser_data.delete_tag(TEST_TAG_NAME).unwrap().1,
            TEST_BASE_ADDR
        );

        // Check that the tag no longer exists in the table.
        assert_eq!(test_parser_data.tag_db.get(TEST_TAG_NAME), None);
    }
} // mod tests
