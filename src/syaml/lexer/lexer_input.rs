pub trait LexerInput {

    const EOF_CHAR: isize = -1;

    /// The current code point character in the input (or LexerInput#Eof if the EoF was reached).
    fn current(&self) -> isize;

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
    fn consume(&self, n: Option<isize>) -> ();

    /// Consume while the condition holds.
    fn consume_while<F>(&self, p: F) -> () where F: Fn(isize) -> bool {
        while p(self.current()) {
            self.consume(None)
        }
    }

    /// Create a mark in the Input so you can reset the input to it later
    fn create_mark<T : Mark>(&self) -> T;

    /// Reset the input to the specified offset
    fn reset(&self, mark: &dyn Mark) -> ();

    /// Return the character `i` characters ahead of the current position, (or LexerInput#Eof if the EoF was reached).
    fn look_ahead(&self, i: isize) -> isize;

    /// Return the sub-sequence of characters between the specified positions
    // fn sub_sequence(&self, start: isize, end: isize) -> CharSequence;

    /// Return the name of the source, if existent (Usually a file name, or similar).
    fn source_name(&self) -> String { String::from("") }

    /// We're not at the Eof
    fn non_eof(&self) -> bool;
}

trait Mark {}