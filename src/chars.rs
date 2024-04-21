pub struct Chars<T> {
    iter: T,
}

impl<T: Iterator<Item = u8>> From<T> for Chars<T> {
    fn from(iter: T) -> Self {
        Self { iter }
    }
}

impl<T: Iterator<Item = u8>> Iterator for Chars<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // Create a variable for the character, preset it to 0
        let mut value = 0;

        // Iterate through at most 4 bytes from the iterator
        for byte in self.iter.by_ref().take(4) {
            // Store the byte
            value = (value << 8) | u32::from(byte);

            // Check whether the number is a valid character, return it if it is
            if let Some(ch) = char::from_u32(value) {
                return Some(ch);
            }
        }

        // The end of the iterator was reached or an invalid character was found
        None
    }
}
