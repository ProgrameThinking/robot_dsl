use lalrpop_util::lalrpop_mod;
use robot_dsl::lexer::Lexer;
use robot_dsl::ast;
//use robot_dsl::interpreter;
lalrpop_mod!(pub grammar);

//#[cfg(not(test))]
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let source_code = std::fs::read_to_string("examples/simple.dsl")?;
    let lexer = Lexer::new(&source_code[..]);
    let parser = grammar::ProgramParser::new();
    let ast = parser.parse(lexer)?;
    for statement in ast {
        let statement_ref: &ast::Statement = statement.as_ref();
        println!("{:?}",statement_ref);

    }
    //let interpreter= Interpreter(ast);
    //interpreter.execute();
    Ok(())
}


/*
 * 由于使用了lalrpop生成parser，因此不独立出文件对parser进行单元测试
 * 测试内容位于main.rs之下
 */


#[cfg(test)]
mod tests {

    use robot_dsl::{ast, lexer::Lexer, lexer::LexicalError,tokens::Token};
    use lalrpop_util::{lalrpop_mod, ParseError};
    lalrpop_mod!(pub grammar);

    pub fn parse_program(
        input: &str,
    ) -> Result<Vec<Box<ast::Statement>>, ParseError<usize, Token, LexicalError>> {
        let lexer = Lexer::new(input);
        let parser = grammar::ProgramParser::new();
        match parser.parse(lexer) {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }

    #[test]
    fn test_global_variable_declaration() {
        let program_str = "global my_var = 42;";
        let ast_result = parse_program(program_str);

        // 检查解析结果是否是 Ok 类型
        if let Ok(ast) = ast_result {
            let expected_ast = vec![Box::new(ast::Statement::Var {
                name: String::from("my_var"),
                init: Box::new(ast::Expr::Literal {
                    value: ast::LiteralValue::Number(42.0),
                }),
            })];
            assert_eq!(ast, expected_ast);
        } else {
            // 如果解析失败，可以选择 panic 或者输出错误信息
            panic!("Failed to parse program: {:?}", ast_result);
        }
    }

    #[test]
    fn test_speak_statement() {
        let program_str = "speak \"Hello World\";";
        let ast_result = parse_program(program_str);
    
        // 检查解析结果是否是 Ok 类型
        if let Ok(ast) = ast_result {
            let expected_ast = vec![Box::new(ast::Statement::Speak {
                expression: Box::new(ast::Expr::Literal {
                    value: ast::LiteralValue::String("Hello World".to_string()),
                }),
            })];
            assert_eq!(ast, expected_ast);
        } else {
            panic!("Failed to parse program: {:?}", ast_result);
        }
    }

    #[test]
    fn test_input_statement() {
        let program_str = "input my_var;";
        let ast_result = parse_program(program_str);
    
        if let Ok(ast) = ast_result {
            let expected_ast = vec![Box::new(ast::Statement::Input {
                input: String::from("my_var"),
            })];
            assert_eq!(ast, expected_ast);
        } else {
            panic!("Failed to parse program: {:?}", ast_result);
        }
    }
    
    #[test]
    fn test_exit_statement() {
        let program_str = "exit;";
        let ast_result = parse_program(program_str);
    
        if let Ok(ast) = ast_result {
            let expected_ast: Vec<Box<ast::Statement>> = vec![Box::new(ast::Statement::Exit)];
            assert_eq!(ast, expected_ast);
        } else {
            panic!("Failed to parse program: {:?}", ast_result);
        }
    }
    
    #[test]
    fn test_if_statement() {
        let program_str = "if (x == 42) { speak \"Hello World\"; };";
        let ast_result = parse_program(program_str);
    
        if let Ok(ast) = ast_result {
            let expected_ast: Vec<Box<ast::Statement>> = vec![Box::new(ast::Statement::Branch {
                condition: Box::new(ast::Expr::Binary {
                    left: Box::new(ast::Expr::Variable {
                        name: String::from("x"),
                    }),
                    operator: Token::WEqual,
                    right: Box::new(ast::Expr::Literal {
                        value: ast::LiteralValue::Number(42.0),
                    }),
                }),
                then: Box::new(ast::Statement::Block {
                    statements: vec![Box::new(ast::Statement::Speak {
                        expression: Box::new(ast::Expr::Literal {
                            value: ast::LiteralValue::String("Hello World".to_string()),
                        }),
                    })],
                }),
            })];
            assert_eq!(ast, expected_ast);
        } else {
            panic!("Failed to parse program: {:?}", ast_result);
        }
    }

    #[test]
    fn test_complex_assignment_expression() {
        let program_str = "global x = (a + 3) * (b - 5);";
        let ast_result = parse_program(program_str);
    
        if let Ok(ast) = ast_result {
            let expected_ast: Vec<Box<ast::Statement>> = vec![Box::new(ast::Statement::Var {
                name: String::from("x"),
                init: Box::new(ast::Expr::Binary {
                    left: Box::new(ast::Expr::Binary {
                        left: Box::new(ast::Expr::Variable {
                            name: String::from("a"),
                        }),
                        operator: Token::OperatorAdd,
                        right: Box::new(ast::Expr::Literal {
                            value: ast::LiteralValue::Number(3.0),
                        }),
                    }),
                    operator: Token::OperatorMul,
                    right: Box::new(ast::Expr::Binary {
                        left: Box::new(ast::Expr::Variable {
                            name: String::from("b"),
                        }),
                        operator: Token::OperatorSub,
                        right: Box::new(ast::Expr::Literal {
                            value: ast::LiteralValue::Number(5.0),
                        }),
                    }),
                }),
            })];
            assert_eq!(ast, expected_ast);
        } else {
            panic!("Failed to parse program: {:?}", ast_result);
        }
    }

    #[test]
    fn test_loop_statement() {
        let program_str = "loop { speak 42; }";
        let ast_result = parse_program(program_str);
    
        if let Ok(ast) = ast_result {
            let expected_ast: Vec<Box<ast::Statement>> = vec![Box::new(ast::Statement::Loop {
                body: Box::new(ast::Statement::Block {
                    statements: vec![Box::new(ast::Statement::Speak {
                        expression: Box::new(ast::Expr::Literal {
                            value: ast::LiteralValue::Number(42.0),
                        }),
                    })],
                }),
            })];
            assert_eq!(ast, expected_ast);
        } else {
            panic!("Failed to parse program: {:?}", ast_result);
        }
    }

}
