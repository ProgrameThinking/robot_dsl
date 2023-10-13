use std::fmt;  // to implement the Display trait
use logos::Logos;

/*
 * 对词素类型进行枚举定义
 * 分为以下三类：
 * keywords
 * signle token class
 * literals
 * 采用了logos库，直接返回分析后的token list
 */
#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
    //keywords
    #[regex(r"(?i)global")]
    KeywordGlobal,
    #[regex(r"(?i)fn")]
    KeywordFn,
    #[regex(r"(?i)speak")]
    KeywordSpeak,
    #[regex(r"(?i)input")]
    KeywordInput,
    #[regex(r"(?i)if")]
    KeywordIf,
    #[regex(r"(?i)exit")]
    KeywordExit,
    #[regex(r"(?i)loop")]
    KeywordLoop,

    //signle token class
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("=")]
    Assign,
    #[token(";")]
    Semicolon,
    #[token("{")]
    LBracket,
    #[token("}")]
    RBracket,

    //operator
    #[token("+")]
    OperatorAdd,
    #[token("-")]
    OperatorSub,
    #[token("*")]
    OperatorMul,
    #[token("/")]
    OperatorDiv,

    //literals
    #[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().parse())]
    Identifier(String),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>())]
    Number(f64),
    #[regex(r#""[^"]*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    StringContent(String),

    //whitespace
    #[regex(r"#.*\n?", logos::skip)]
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    #[error]
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
    }
  }