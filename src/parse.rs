use lex::Lex;
use lex::Token;
use std::result;
use std::iter::Peekable;

type Result<T> = result::Result<T, String>;

#[derive(Debug)]
pub struct Invocation {
    command: Invocable,
    list: Vec<Token>
}

#[derive(Debug)]
pub struct Invocable {
    token: Token
}

//
// pub struct
//
// pub enum ASTNode {
//     Invocation(Box<ASTNode>, Box<ASTNode>),
//     Invocable(Token),
//     Expression(Option<Vec<Token>>)
// }

// macro_rules! expect {
//     ($x:expr, $y:path) => {{
//         match $x {
//             Some(token @ $y) => Ok(token),
//             Some(token) => Err(format!("Unexpected {}", token)),
//             None => Err(format!("Unexpected end of input"))
//         }
//     }}
// }

pub struct Parse<'a> {
    token_stream: Peekable<Lex<'a>>
}

impl<'a> Parse<'a> {
    pub fn new(tokenizer: Lex) -> Parse {
        Parse {token_stream: tokenizer.peekable()}
    }

    pub fn parse(&mut self) -> Result<Invocation> {
        let invocation = self.parse_invocation()?;

        // token stream should be empty, or there's unexpected garbage at end of input
        if let Some(token) = self.token_stream.peek() {
            return Err(format!("Unexpected {:?}", token));
        }

        return Ok(invocation);
    }

    fn parse_invocation(&mut self) -> Result<Invocation> {
        let invocable  = self.parse_invocable()?;
        let arguments = self.parse_list()?;
        Ok(Invocation{invokee: invocable, expression: arguments})
    }

    fn parse_invocable(&mut self) -> Result<Invocable> {
        match self.token_stream.next() {
            Some(token @ Token::Morpheme(_)) => Ok(Invocable{token: token}),
            Some(other) => return Err(format!("Unexpected {:?}", other)),
            None => return Err(format!("Unexpected end of input"))
        }
    }

    fn parse_list(&mut self) -> Result<Vec<Token>> {
        let mut expr = Vec::new();

        loop {
             match self.token_stream.peek() {
                Some(&Token::Morpheme(_)) | Some(&Token::Str(_)) => {
                    if let Some(token) = self.token_stream.next() {
                        expr.push(token)
                    }
                },
                _ => return Ok(expr)
            }
        }
    }
}

#[test]
fn test_parse() {
    let input = "ls -al";
    let parser = Parse::new(Lex::new(input));
    use lex::Token::*;
    use parse::ASTNode::*;

//    assert_eq!(Ok(Invocation(Invocable(Morpheme("ls")), Expression(Some([Morpheme("-al")])))), parser.parse());
}
