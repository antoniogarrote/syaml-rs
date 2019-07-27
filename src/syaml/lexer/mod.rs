mod input_range;
pub use self::input_range::InputRange;

struct Token {
    name: String,
    abbrevation: String
}

struct TokenData<T> {
    token: T,
    range: InputRange,
    start: usize,
    end: usize
}

struct AstToken {
    token_type: Token,
    text: String,
    range: InputRange,
    parsing_error: bool
}