pub trait LexerInput {

    const EOF_CHAR: isize = -1;

    /// The current code point character in the input (or LexerInput#Eof if the EoF was reached).
    fn current(&self) -> Option<char>;

    /// The absolute offset (0..n) of the current character.
    fn offset(&self) -> isize;

    /// The index of the character relative to the beginning of the line, as a 16 bit java character. (0 based)
    fn column(&self) -> isize;

    // The current Line number (0 based).
    fn line(&self) -> isize;

    /// the triple (line, column, offset)
    fn position(&self) -> (isize, isize, isize);

    /// Consume and advance to the next code point.
    /// Consume n code points
    fn consume(&self, n: Option<usize>) -> ();

    /// Consume while the condition holds.
    fn consume_while<F>(&self, p: F) -> () where F: Fn(char) -> bool {
        let next_char = self.current();
        while next_char.is_some() && p(next_char.unwrap()) {
            self.consume(None)
        }
    }

    /// Create a mark in the Input so you can reset the input to it later
    fn create_mark<'a>(&self) -> Mark;

    /// Reset the input to the specified offset
    fn reset(&self, mark: Mark) -> ();

    /// Return the character `i` characters ahead of the current position, (or LexerInput#Eof if the EoF was reached).
    fn look_ahead(&self, i: usize) -> char;

    /// Return the sub-sequence of characters between the specified positions
    // fn sub_sequence(&self, start: isize, end: isize) -> CharSequence;

    /// Return the name of the source, if existent (Usually a file name, or similar).
    fn source_name(&self) -> String { String::from("") }

    /// We're not at the Eof
    fn non_eof(&self) -> bool;
}

pub struct Mark {}