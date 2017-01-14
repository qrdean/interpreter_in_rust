// Modified interprepreter in rust
// Based off https://github.com/chr4/writing_an_interpreter_in_rust.git
// & https://github.com/rspivak/lsbasi.git
use std::str::Chars;
use std::iter::Peekable;

use std::io::{self, BufRead, Write};
// TokenTypes
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    // Type
    Integer(i32),

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,

    // Eof and Illegal Tokens
    Illegal,
    Eof,
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal
    }
}

pub struct Interpreter<'a> {
    // Peekable is an iter that gives an option to peek without iterating
    input: Peekable<Chars<'a>>,
    current_token: Token,
}

impl<'a> Interpreter<'a> {
    pub fn new(input: &str) -> Interpreter {
        Interpreter {
            input: input.chars().peekable(),
            current_token: Token::Illegal
        }
    }

    /////////////////////
    // Lexer Functions //
    /////////////////////

    fn error(&self) {
        println!("Invalid syntax"); // TODO: make meaningful error messages
    }

    // Goes to the next char
    fn advance(&mut self) -> Option<char> {
        self.input.next()
    }

    // "Peeks" at the next char w/o iterating
    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek_char() {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    // Enables multi-char integers and converts to i32
    fn integer(&mut self, first: char) -> i32 {
        let mut number_str = String::new();
        number_str.push(first);

        while let Some(&ch) = self.peek_char() {
            if !ch.is_numeric() {
                break;
            }
            number_str.push(self.advance().unwrap());
        }
        let number:i32 = number_str.parse().unwrap();
        number
    }

    /*  Lexical analyzer

    This function breaks a sentence apart into tokens, one at a time.
    */
    fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.advance() {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Multiply,
            Some('/') => Token::Divide,

            Some(ch @ _) => {
                if ch.is_numeric(){
                    Token::Integer(self.integer(ch))
                } else {
                    Token::Illegal
                }
            }
            None => Token::Eof,
        }
    }

    // Peeks at the next token (end-of-file)
    fn peek_next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.peek_char() {
            Some(&'+') => Token::Plus,
            Some(&'-') => Token::Minus,
            Some(&'*') => Token::Multiply,
            Some(&'/') => Token::Divide,

            Some(&ch @ _) => {
                if ch.is_numeric(){
                    Token::Integer(self.integer(ch))
                } else {
                    Token::Illegal
                }
            }
            None => Token::Eof,
        }
    }

    ///////////////////////////
    // Interpreter Functions //
    ///////////////////////////


    // compare the current token type w/ the passed token type
    // and if they match then "eat" the current token and assign
    // the next token to the self.current_token, otherwise "Error"
    fn eat(&mut self, token_type: Token) {
        if self.current_token == token_type {
            self.current_token = self.get_next_token();
        } else {
            self.error();
        }
    }

    // "pull" i32 out of the token_type Integer(i32)
    // if it's not an integer return 0
    fn process_token(&mut self) -> i32 {
        match self.current_token {
            Token::Integer(num) => num,
            _                   => 0,
        }
    }

    // encapsulation of a "term"
    // checks to make sure is Token::Integer(i32)
    fn term(&mut self) -> i32 {
        let result = self.process_token();
        self.eat(Token::Integer(result));
        result
    }

    // Arithmetic expression parser
    fn expr(&mut self) -> i32 {
        // set current token to the first token taken from the input
        self.current_token = self.get_next_token();
        let mut result = self.term();
        loop {
            // Set Operator '+' or '-' and match
            // if neither then break
            let token = self.current_token;
            match token {
                Token::Plus => {
                    self.eat(Token::Plus);
                    result = result + self.term();
                },

                Token::Minus => {
                    self.eat(Token::Minus);
                    result = result - self.term();
                },

                _           => { break; }
            }
        }
        result
    }
}

fn main() {
    let stdin = io::stdin();

    loop {
        // Stdout needs to be flushed, due to missing newline
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");
        let mut interpreter = Interpreter::new(&mut line);

        loop {
            //let tok = interpreter.get_next_token();
            //println!("{:?}", tok);
            let result = interpreter.expr();
            println!("{}",result);
            if interpreter.peek_next_token() == Token::Eof {
                break;
            }
        }
    }
}
