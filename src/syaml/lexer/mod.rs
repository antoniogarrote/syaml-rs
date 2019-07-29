mod input_range;
mod lexer_input;
mod queue;

pub use self::input_range::InputRange;
pub use self::lexer_input::LexerInput;

trait Token {
    fn name(&self) -> &String;
    fn abbreviation(&self) -> &String;
}

struct TokenData<T : Token> {
    token: T,
    range: InputRange,
    start: isize,
    end: isize
}

impl <T: Token> TokenData<T> {
    /// Computes the range between two tokens
    pub fn range_to(&self, to:&TokenData<T>) -> InputRange {
        InputRange::build(self.range.line_from, self.range.column_from, to.range.line_to, to.range.column_to)
    }
}

struct AstToken<T: Token> {
    token_type: T,
    text: String,
    range: InputRange,
    parsing_error: bool
}

trait Lexer<T: Token> {
    /// get the current token in the input stream.
    fn token(&self) -> T;

    /// All the token data.
    fn token_data(&self) -> TokenData<T>;

    /// Advance the lexer to the next token.
    fn advance(&self) -> ();

    /*
    /// Get the current Token Char Sequence.
    fn tokenText() -> CharSequence

    /// Get the current Token String.
    fn tokenString() -> String = tokenText().toString
    */
}


#[cfg(test)]
mod tests {

    use super::TokenData;
    use crate::syaml::lexer::{Token, InputRange};

    struct TestToken {
        name: String,
        abbreviation: String
    }

    impl Token for TestToken {
        fn name(&self) -> &String {
            &self.name
        }

        fn abbreviation(&self) -> &String {
            &self.abbreviation
        }
    }

    #[test]
    fn test_token_data_range() {
        let token1 = TestToken{
            name: String::from("Token1"),
            abbreviation: String::from("t1")
        };
        let range1 = InputRange::build(1,1,10,1);
        let tdata1 = TokenData{
            token: token1,
            range: range1,
            start: 0,
            end: 10
        };

        let token2 = TestToken{
            name: String::from("Token2"),
            abbreviation: String::from("t2")
        };
        let range2 = InputRange::build(100,1,1000,1);
        let tdata2 = TokenData{
            token: token2,
            range: range2,
            start: 0,
            end: 10
        };

        let range = tdata1.range_to(&tdata2);
        assert_eq!(range.line_from, tdata1.range.line_from);
        assert_eq!(range.column_from, tdata1.range.column_from);
        assert_eq!(range.line_to, tdata2.range.line_to);
        assert_eq!(range.column_to, tdata1.range.column_to);
    }
}