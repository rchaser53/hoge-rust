#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TokenType {
  Identifier,
  Digit,
  
  Eof,
  Assign,
  Colon,
  Comma,
  Period,
  Semicolon,
  
  Lparen,          // (
  Rparen,          // )
  Lbrace,          // {
  Rbrace,          // }

  // preserve word
  Fn,              // fn
  True,            // true
  False,           // false
  If,              // if
  Else,            // else
  Let,             // let
  Return,          // return

  // for Arithmetic
  Eq,              // =
  NotEq,           // !=
  Lt,              // <
  Lte,             // <=
  Gt,              // >
  Gte,             // =>
  Plus,            // +
  Minus,           // -
  Slash,           // /
  Asterisk,        // *
  Bang,            // !
}

#[derive(Clone, Debug)]
pub struct Token {
  pub kind: TokenType,
  pub value: String
}

impl Token {
  pub fn new(kind: TokenType, value: String) -> Token {
    Token {
      kind: kind,
      value: value
    }
  }
}

impl PartialEq for Token {
  fn eq(&self, other: &Token) -> bool {
      self.kind == other.kind && self.value == other.value
  }
}