use super::tokens::Token;
use logos::{Logos, SpannedIter};
use std::fmt; // your enum

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug)]
pub enum LexicalError {
    InvalidToken,
}
impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexicalError::InvalidToken => write!(f, "Invalid token encountered."),
        }
    }
}
pub struct Lexer<'input> {
    // instead of an iterator over characters, we have a token iterator
    token_stream: SpannedIter<'input, Token>,
}
impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}
impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| {
            match token {
                // an invalid token was met
                Token::Error => Err(LexicalError::InvalidToken),
                _ => Ok((span.start, token, span.end)),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::tokens::Token;

    // 编写测试函数来测试解析规则
    fn test_token(input: &str, expected_token: Token) {
        let mut lexer = Lexer::new(input);
        if let Some(Ok((_, token, _))) = lexer.next() {
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_keywords_lexing() {
        test_token("global", Token::KeywordGlobal);
        test_token("fn", Token::KeywordFn);
        test_token("speak", Token::KeywordSpeak);
        test_token("input", Token::KeywordInput);
        test_token("if", Token::KeywordIf);
        test_token("exit", Token::KeywordExit);
    }

    #[test]
    fn test_single_lexing() {
        test_token("(", Token::LParen);
        test_token(")", Token::RParen);
        test_token("=", Token::Assign);
        test_token(";", Token::Semicolon);
        test_token("{", Token::LBracket);
        test_token("}", Token::RBracket);
    }

    #[test]
    fn test_operators_lexing() {
        test_token("+", Token::OperatorAdd);
        test_token("-", Token::OperatorSub);
        test_token("*", Token::OperatorMul);
        test_token("/", Token::OperatorDiv);
    }

    #[test]
    fn test_literals_lexing() {
        //identifier test
        test_token(
            "identifier123",
            Token::Identifier("identifier123".to_string()),
        );
        test_token(
            "identifier123",
            Token::Identifier("identifier123".to_string()),
        );
        test_token("_underscore", Token::Identifier("_underscore".to_string()));
        test_token("CamelCase", Token::Identifier("CamelCase".to_string()));
        test_token(
            "leadingDigit123",
            Token::Identifier("leadingDigit123".to_string()),
        );
        test_token(
            "mixed_Case123",
            Token::Identifier("mixed_Case123".to_string()),
        );

        //number test
        test_token("123.456", Token::Number(123.456));
        test_token("0.5", Token::Number(0.5));
        test_token("1.0", Token::Number(1.0));
        test_token("2.5", Token::Number(2.5));
        //string test
        test_token(
            "\"This is a string\"",
            Token::StringContent("This is a string".to_string()),
        );
        test_token(
            "\"Empty string: \"\"",
            Token::StringContent("Empty string: ".to_string()),
        );

        //Chinese test
        test_token("这是中文", Token::Identifier("这是中文".to_string()));
        test_token(
            "中文标识符123",
            Token::Identifier("中文标识符123".to_string()),
        );
        test_token("数字：123", Token::Identifier("数字：123".to_string()));
        test_token(
            "字符串：\"中文字符串\"",
            Token::StringContent("中文字符串".to_string()),
        );
    }

    #[test]
    fn test_program_lexing() -> Result<(), Box<dyn std::error::Error>> {
        let input_program = std::fs::read_to_string("examples/simple.dsl")?;

        let mut lexer = Lexer::new(&input_program[..]);
        let expected_tokens = vec![
            Token::KeywordGlobal,
            Token::Identifier("name".to_string()),
            Token::Assign,
            Token::StringContent("".to_string()),
            Token::Semicolon,
            Token::KeywordFn,
            Token::Identifier("test".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBracket,
            Token::KeywordSpeak,
            Token::StringContent("123321".to_string()),
            Token::Semicolon,
            Token::RBracket,
            Token::KeywordLoop,
            Token::LBracket,
            Token::KeywordSpeak,
            Token::StringContent("321123".to_string()),
            Token::Semicolon,
            Token::KeywordExit,
            Token::Semicolon,
            Token::RBracket,
        ];

        for expected_token in expected_tokens {
            let token = lexer.next().unwrap().unwrap().1;
            assert_eq!(token, expected_token);
        }

        Ok(())
    }
}
