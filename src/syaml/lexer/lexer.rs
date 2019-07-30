use crate::syaml::lexer::{Token, TokenData, Position, InputRange, LexerInput};
use std::str::Chars;
use crate::syaml::lexer::lexer_input::Mark;
use crate::syaml::lexer::queue::Queue;

pub struct LexerState {
    queue_size: usize,
    position: Position,
    mark: Mark
}

pub trait Lexer<T: Token, L: LexerInput> {

    fn input(&self) -> &L;

    /// get the current token in the input stream.
    fn token(&self) -> T;

    /// Marks a position
    fn mark(&self) -> Position;

    /// Returns the current mark
    fn set_mark(&self, mark: Position) -> ();

    /// TokenQueue for the lexer
    fn token_queue(&self) -> &mut Queue<TokenData<T>>;

    /// All the token data.
    fn token_data(&self) -> TokenData<T>;

    fn set_token_data(&self, token_data: TokenData<T>) -> ();

    /// Advance the lexer to the next token.
    fn advance(&self) -> () {
        let _input = self.input();

        while self.non_token_emitted() {
            let current = self.current_char();
            if current.is_some() {
                let pos = self.input().offset();
                self.find_token(current.unwrap());
                if pos == self.input().offset() {
                    self.advance()
                }
            } else {
                self.process_pending()
            }
        }
        let head = self.token_queue().dequeue();
        self.set_token_data(head)
    }

    /// Returns the position of the lexer
    fn position(&self) -> Position;

    /// True if no token has been emitted yet
    fn non_token_emitted(&self) -> bool;

    /// Get the current Token Char Sequence.
    fn token_text(&self) -> &Chars;

    /// Get the current Token String.
    fn token_string(&self) -> &str;

    /// Emits a new token for the marked region and updates the mark
    fn emit1(&self, token: T) -> bool {
        let mark = self.mark();
        let new_mark = self.position();
        let queue = self.token_queue();
        let range = InputRange::build(mark.line, mark.column, new_mark.line, new_mark.column);
        let token_data = TokenData {
            token,
            range,
            start: mark.position,
            end: new_mark.position
        };
        queue.append(token_data);
        self.set_mark(new_mark);
        true
    }

    /// Emits 2 tokens
    fn emit2(&self, token1: T, token2: T) -> bool {
        self.emit1(token1);
        self.emit1(token2)
    }

    fn emit2_for_mark(&self, token1: T, token2: T) -> bool {
        let initial_mark = self.mark();
        self.emit1(token1);
        self.set_mark(initial_mark);
        self.emit1(token2)
    }

    fn reset(&self) {
        let current_position = self.position();
        self.set_mark(current_position)
    }

    fn find_token(&self, chr: char) -> ();

    fn current_char(&self) -> Option<char> {
        self.input().current()
    }


    /// Process all pending tokens. Trivial implementation just emit the EofToken
    /// More complex ones can continue returning pending tokens until they emit the EofToken
    fn process_pending(&self) -> ();


    /// Return the character `i` characters ahead of the current position, (or LexerInput#Eof if the EoF was reached).
    fn look_ahead(&self, n: usize) -> char {
        self.input().look_ahead(n)
    }

    /// Consume and advance to the next code point.
    /// Consume n code points
    fn consume(&self, n: Option<usize>)-> () {
        self.input().consume(n);
    }

    /// Consume while the condition holds.
    fn consume_while<F>(&self, p: F) -> () where F: Fn(char) -> bool {
        self.input().consume_while(p)
    }

    /// Compare with the specified char and consume if they are equal
    fn consume_char(&self, c: char) -> bool {
        match self.current_char() {
            Some(current) if current == c => {
                self.consume(None);
                true
            }
            _ => false
        }
    }

    /// Compare with the specified String and consume if all characters are equal
    fn consume_string(&self, input: &String) -> bool {
        let l = self.check(input);
        if l == 0 {
            false
        } else {
            for c in input.chars() {
                if !self.consume_char(c) {
                    return false;
                }
            }
            true
        }
    }

    /// Compare with the specified String and return 0 or the length of the string if all characters
    /// are equal
    fn check(&self, str: &String) -> usize {
        let len = str.len();
        for (i,c) in str.chars().enumerate() {
            if self.look_ahead(i) != c {
                return  0;
            }
        }
        return len;
    }

    fn consume_and_emit(&self, n: Option<usize>, token: T) -> bool {
        self.consume(n);
        self.emit1(token)
    }

    fn consume_and_emit2(&self, token1: T, token2: T) -> bool {
        self.consume(None);
        self.emit1(token1);
        self.emit1(token2)
    }

    /// We're not at the Eof
    fn non_eof(&self) -> bool {
        self.input().non_eof()
    }

    fn matches<F>(&self, p: F) -> bool where F: Fn() -> bool {
        let s = self.save_state();
        let result = p();
        if !result { self.restore_state(s); }
        result
    }

    fn zero_or_more<F>(&self, p: F) -> bool where F: Fn() -> bool {
        let mut s = self.save_state();
        while self.non_eof() && p() {
            s = self.save_state()
        }
        self.restore_state(s);
        true
    }

    fn one_or_more<F>(&self, p: F) -> bool where F: Fn() -> bool {
        let mut s = self.save_state();
        let result = p();
        if result {
            while self.non_eof() && p() {
                s = self.save_state();
            }
        }
        self.restore_state(s);
        result
    }

    fn save_state(&self) -> LexerState {
        let queue_size = self.token_queue().size();
        let mark = self.mark();
        let input_mark: Mark = self.input().create_mark();
        LexerState {
            queue_size,
            position: mark,
            mark: input_mark
        }
    }

    fn restore_state(&self, lexer_state: LexerState) -> () {
        self.token_queue().reduce_to(lexer_state.queue_size);
        self.set_mark(lexer_state.position);
        self.input().reset(lexer_state.mark)
    }

    fn begin_of_line(&self) -> bool {
        self.input().column() == 0
    }

    fn optional(_: bool) -> bool  {
        true
    }
}