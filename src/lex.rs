use itertools;
use itertools::Itertools;
use itertools::PutBack;
use std::str::Chars;

const STRING_DELIM: char = '"';
const PIPE_DELIM: char = '|';

#[derive(Debug, PartialEq)]
pub enum Token {
    Morpheme(String),
    Str(String),
    Pipe,
    UnterminatedString,
}

pub struct Lex<'a> {
    iter: PutBack<Chars<'a>>,
}

impl<'a> Lex<'a> {
    pub fn new(input: &str) -> Lex {
        Lex {
            iter: itertools::put_back(input.chars()),
        }
    }

    pub fn produce(&mut self) -> Option<Token> {
        if let Some(c) = self.next_non_whitespace() {
            let token = match c {
                PIPE_DELIM => Token::Pipe,
                STRING_DELIM => self.produce_string(),
                _ => {
                    self.put_back(c);
                    self.produce_morpheme()
                }
            };
            return Some(token);
        } else {
            return None;
        }
    }

    fn put_back(&mut self, c: char) {
        self.iter.put_back(c);
    }

    fn next_non_whitespace(&mut self) -> Option<char> {
        // drain whitespace
        self.iter
            .take_while_ref(|c| c.is_whitespace())
            .foreach(|_| ());
        self.iter.next()
    }

    fn produce_string(&mut self) -> Token {
        let string = self.iter.take_while_ref(|c| c != &STRING_DELIM).collect();

        // ensure next character is ", and that we didn't hit None,
        // otherwise we have an UnterminatedString
        match self.iter.next() {
            Some('"') => Token::Str(string),
            _ => Token::UnterminatedString,
        }
    }

    fn produce_morpheme(&mut self) -> Token {
        let morpheme_chars = self.iter.take_while_ref(|c| !c.is_whitespace());
        Token::Morpheme(morpheme_chars.collect())
    }
}

impl<'a> Iterator for Lex<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.produce()
    }
}

#[test]
fn basic_parse() {
    let mut lexer =
        Lex::new("foo -m \"a long string with       stuff\" -a bar | baz \"unterminated string");

    let expectations = [
        Token::Morpheme("foo".to_owned()),
        Token::Morpheme("-m".to_owned()),
        Token::Str("a long string with       stuff".to_owned()),
        Token::Morpheme("-a".to_owned()),
        Token::Morpheme("bar".to_owned()),
        Token::Pipe,
        Token::Morpheme("baz".to_owned()),
        Token::UnterminatedString,
    ];

    for expected in expectations.iter() {
        if let Some(tok) = lexer.next() {
            assert_eq!(*expected, tok);
        } else {
            // we got None before all expectations were exhausted
            assert!(false);
        }
    }
    // ensure the lex doesn't produce extra tokens
    assert_eq!(lexer.produce(), None);
}
