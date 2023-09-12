use hex;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /**
     * Generate a color based on the hash of a string.
     */
    pub fn from_text(text: String) -> Self {
        let mut new_text = text.clone();

        // Check that the text has at least 3 characters (since the numbe of bytes
        // is doubled when converting to hex)
        // If not, we just double the text until we have at least 3 characters
        while new_text.len() < 3 {
            new_text = format!("{0}{0}", new_text);
        }

        // We hash the text
        let hashed_text = hex::encode(new_text);

        // We take the first 6 characters of the hash
        // We can `unwrap` because we know that the hash is at least 6 characters long
        // AND that each character is a valid hex character (0-9, A-F, a-f)
        let r = u8::from_str_radix(&hashed_text[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hashed_text[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hashed_text[4..6], 16).unwrap();

        Self { r, g, b }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        self.to_hex()
    }
}
