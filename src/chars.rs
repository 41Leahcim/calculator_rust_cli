pub struct Chars<T> {
    iter: T,
}

impl<T: Iterator<Item = u8>> Chars<T> {
    pub const fn new(iter: T) -> Self {
        Self { iter }
    }
}

impl<T: Iterator<Item = u8>> Iterator for Chars<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value = 0;
        for byte in self.iter.by_ref() {
            value = (value << 8) + u32::from(byte);
            if let Some(ch) = char::from_u32(value) {
                return Some(ch);
            }
        }
        None
    }
}
